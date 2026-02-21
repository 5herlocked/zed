use anyhow::Result;
use bytes::Bytes;
use clap::Parser;
use futures::stream::StreamExt;
use futures::{SinkExt, TryStreamExt};
use git::GitHostingProviderRegistry;
use gpui::{AppContext as _, display_tree::DisplayTree};
use gpui::{Application, PlatformInput, StreamingConfig};
use language::LanguageRegistry;
use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch as tokio_watch;
use tokio_tungstenite::tungstenite::Message;
use workspace::AppState;

mod input_parser;

#[derive(Parser)]
#[command(name = "zed_server", about = "Zed Web streaming server")]
struct Args {
    /// WebSocket bind address
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    bind: SocketAddr,

    /// Viewport width in logical pixels
    #[arg(long, default_value_t = 1280.0)]
    width: f32,

    /// Viewport height in logical pixels
    #[arg(long, default_value_t = 720.0)]
    height: f32,

    /// Display scale factor
    #[arg(long, default_value_t = 2.0)]
    scale: f32,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let config = StreamingConfig {
        width: args.width,
        height: args.height,
        scale_factor: args.scale,
    };

    let (app, frame_rx, resize_tx) = Application::streaming(config);

    let bind_addr = args.bind;
    // Watch channel holds the latest frame -- new clients get it immediately.
    let (frame_watch_tx, frame_watch_rx) = tokio_watch::channel(Bytes::new());

    app.run(move |cx| {
        // ── 1. Core infrastructure ──────────────────────────────────────
        release_channel::init(semver::Version::new(0, 1, 0), cx);
        gpui_tokio::init(cx);
        settings::init(cx);

        // ── 2. HTTP client ──────────────────────────────────────────────
        let http = {
            let _guard = gpui_tokio::Tokio::handle(cx).enter();
            reqwest_client::ReqwestClient::proxy_and_user_agent(None, "ZedServer/0.1.0")
                .expect("could not start HTTP client")
        };
        cx.set_http_client(Arc::new(http));

        // ── 3. Filesystem ───────────────────────────────────────────────
        let fs = Arc::new(fs::RealFs::new(None, cx.background_executor().clone()));
        <dyn fs::Fs>::set_global(fs.clone(), cx);

        // ── 4. Git hosting ──────────────────────────────────────────────
        GitHostingProviderRegistry::default_global(cx);
        git_hosting_providers::init(cx);

        // ── 5. Client (disconnected -- no auth, just satisfies AppState) ─
        let client = client::Client::production(cx);
        client::Client::set_global(client.clone(), cx);

        // ── 6. Language registry (empty -- no built-in languages) ────────
        let mut languages = LanguageRegistry::new(cx.background_executor().clone());
        languages.set_language_server_download_dir(paths::languages_dir().clone());
        let languages = Arc::new(languages);

        // ── 7. Node runtime (minimal, no shell env) ─────────────────────
        let (_, node_options_rx) = watch::channel(None);
        let node_runtime = node_runtime::NodeRuntime::new(
            client.http_client().clone(),
            None,
            node_options_rx,
        );

        // ── 8. Entity stores ────────────────────────────────────────────
        let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
        let workspace_store = cx.new(|cx| workspace::WorkspaceStore::new(client.clone(), cx));

        // ── 9. Session (test mode -- no SQLite DB needed) ───────────────
        let session = session::Session::test();
        let app_session = cx.new(|cx| session::AppSession::new(session, cx));

        // ── 10. AppState ────────────────────────────────────────────────
        let app_state = Arc::new(AppState {
            languages,
            client: client.clone(),
            user_store,
            fs: fs.clone(),
            build_window_options: |_, _| gpui::WindowOptions::default(),
            workspace_store,
            node_runtime,
            session: app_session,
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        // ── 11. Theme (base only -- no embedded assets) ─────────────────
        theme::init(theme::LoadThemes::JustBase, cx);

        // ── 12. Core crate init (order matters) ─────────────────────────
        project::Project::init(&client, cx);
        client::init(&client, cx);
        editor::init(cx);
        workspace::init(app_state.clone(), cx);

        // ── 13. Open an empty workspace ─────────────────────────────────
        workspace::open_new(
            Default::default(),
            app_state.clone(),
            cx,
            |_workspace, _window, _cx| {},
        )
        .detach();

        // ── 14. Frame bridge + WebSocket server ─────────────────────────
        let tokio = gpui_tokio::Tokio::handle(cx).clone();
        tokio.spawn(frame_bridge(frame_rx, frame_watch_tx));

        let resize_tx_clone = resize_tx.clone();
        let watch_rx = frame_watch_rx;
        tokio.spawn(async move {
            if let Err(e) = run_websocket_server(bind_addr, watch_rx, resize_tx_clone).await {
                error!("WebSocket server error: {e:#}");
            }
        });

        info!("zed_server listening on ws://{bind_addr}");
    });

    Ok(())
}

/// Reads DisplayTree frames from GPUI's smol channel and publishes
/// the latest protobuf-encoded bytes to the watch channel.
async fn frame_bridge(
    frame_rx: smol::channel::Receiver<DisplayTree>,
    watch_tx: tokio_watch::Sender<Bytes>,
) {
    use prost::Message as _;
    loop {
        match frame_rx.recv().await {
            Ok(tree) => {
                let mut buf = Vec::with_capacity(tree.encoded_len());
                match tree.encode(&mut buf) {
                    Ok(()) => {
                        let _ = watch_tx.send(Bytes::from(buf));
                    }
                    Err(e) => error!("frame serialization failed: {e}"),
                }
            }
            Err(_) => {
                info!("frame channel closed, shutting down bridge");
                break;
            }
        }
    }
}

const VIEWER_HTML: &str = include_str!("../viewer.html");

/// Accepts WebSocket connections and serves the viewer HTML for regular HTTP requests.
async fn run_websocket_server(
    addr: SocketAddr,
    watch_rx: tokio_watch::Receiver<Bytes>,
    resize_tx: smol::channel::Sender<(f32, f32, f32)>,
) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server bound to {addr}");

    loop {
        let (stream, peer) = listener.accept().await?;
        info!("new connection from {peer}");
        let rx = watch_rx.clone();
        let rtx = resize_tx.clone();
        tokio::spawn(handle_connection(stream, peer, rx, rtx));
    }
}

/// Peek at the incoming bytes to determine if this is a WebSocket upgrade
/// or a plain HTTP request. Serve viewer.html for the latter.
async fn handle_connection(
    stream: TcpStream,
    peer: SocketAddr,
    watch_rx: tokio_watch::Receiver<Bytes>,
    resize_tx: smol::channel::Sender<(f32, f32, f32)>,
) {
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;

    let mut buf = [0u8; 4096];
    let stream = match stream.peek(&mut buf).await {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buf[..n]);
            if request.contains("Upgrade: websocket") || request.contains("upgrade: websocket") {
                handle_client(stream, peer, watch_rx, resize_tx).await;
                return;
            }
            stream
        }
        Err(e) => {
            error!("peek failed for {peer}: {e}");
            return;
        }
    };

    // Read the HTTP request to consume it from the buffer.
    let mut stream = stream;
    let n = match stream.read(&mut buf).await {
        Ok(n) => n,
        Err(_) => return,
    };
    let _ = n;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        VIEWER_HTML.len(),
        VIEWER_HTML,
    );
    stream.write_all(response.as_bytes()).await.ok();
}

/// Handles a single WebSocket client: sends current + future frames.
async fn handle_client(
    stream: TcpStream,
    peer: SocketAddr,
    mut watch_rx: tokio_watch::Receiver<Bytes>,
    resize_tx: smol::channel::Sender<(f32, f32, f32)>,
) {
    let ws = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("WebSocket handshake failed for {peer}: {e}");
            return;
        }
    };

    let (mut ws_tx, mut ws_rx) = ws.split();

    // Send the latest frame immediately (if available).
    {
        let current = watch_rx.borrow().clone();
        if !current.is_empty() {
            if ws_tx
                .send(Message::Binary(current.into()))
                .await
                .is_err()
            {
                info!("{peer} disconnected during initial send");
                return;
            }
        }
    }

    let peer_send = peer;
    let send_task = tokio::spawn(async move {
        while watch_rx.changed().await.is_ok() {
            let frame = watch_rx.borrow().clone();
            if frame.is_empty() {
                continue;
            }
            if ws_tx.send(Message::Binary(frame.into())).await.is_err() {
                break;
            }
        }
        info!("{peer_send} send loop ended");
    });

    let peer_recv = peer;
    let recv_task = tokio::spawn(async move {
        use gpui::display_tree::{WireFrame, wire_frame};
        use prost::Message as _;
        while let Ok(Some(msg)) = ws_rx.try_next().await {
            let data: Vec<u8> = match msg {
                Message::Binary(data) => data.to_vec(),
                Message::Text(text) => text.as_bytes().to_vec(),
                Message::Close(_) => break,
                _ => continue,
            };
            match WireFrame::decode(data.as_slice()) {
                Ok(frame) => {
                    if let Some(wire_frame::Frame::ViewportChanged(vc)) = frame.frame {
                        let w = vc.width;
                        let h = vc.height;
                        let s = vc.scale_factor;
                        if w > 0.0 && h > 0.0 {
                            info!("{peer_recv} viewport: {w}x{h} @{s}x");
                            let _ = resize_tx.try_send((w, h, s));
                        }
                    }
                }
                Err(e) => {
                    log::debug!("{peer_recv} failed to decode WireFrame: {e}");
                }
            }
        }
        info!("{peer_recv} recv loop ended");
    });

    tokio::select! {
        _ = send_task => {}
        _ = recv_task => {}
    }
    info!("{peer} disconnected");
}

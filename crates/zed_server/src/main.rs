use anyhow::Result;
use bytes::Bytes;
use clap::Parser;
use futures::stream::StreamExt;
use futures::{SinkExt, TryStreamExt};
use git::GitHostingProviderRegistry;
use gpui::{AppContext as _, display_tree::DisplayTree};
use gpui::{Application, StreamingConfig};
use language::LanguageRegistry;
use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch as tokio_watch;
use tokio_tungstenite::tungstenite::Message;
use workspace::AppState;

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
/// the latest serialized JSON to the watch channel.
async fn frame_bridge(
    frame_rx: smol::channel::Receiver<DisplayTree>,
    watch_tx: tokio_watch::Sender<Bytes>,
) {
    loop {
        match frame_rx.recv().await {
            Ok(tree) => {
                match serde_json::to_vec(&tree) {
                    Ok(json) => {
                        let _ = watch_tx.send(Bytes::from(json));
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

/// Accepts WebSocket connections and sends the latest frame + future frames.
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
        tokio::spawn(handle_client(stream, peer, rx, rtx));
    }
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
        while let Ok(Some(msg)) = ws_rx.try_next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(vc) = val.get("ViewportChanged") {
                            let w = vc.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                            let h = vc.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                            let s = vc.get("scale_factor").and_then(|v| v.as_f64()).unwrap_or(2.0) as f32;
                            if w > 0.0 && h > 0.0 {
                                info!("{peer_recv} viewport: {w}x{h} @{s}x");
                                let _ = resize_tx.try_send((w, h, s));
                            }
                        }
                    }
                }
                Message::Binary(data) => {
                    if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&data) {
                        if let Some(vc) = val.get("ViewportChanged") {
                            let w = vc.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                            let h = vc.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                            let s = vc.get("scale_factor").and_then(|v| v.as_f64()).unwrap_or(2.0) as f32;
                            if w > 0.0 && h > 0.0 {
                                info!("{peer_recv} viewport: {w}x{h} @{s}x");
                                let _ = resize_tx.try_send((w, h, s));
                            }
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
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

use anyhow::Result;
use bytes::Bytes;
use clap::Parser;
use futures::stream::StreamExt;
use futures::{SinkExt, TryStreamExt};
use git::GitHostingProviderRegistry;
use gpui::display_tree::{display_action_kind, wire_frame, DisplayTree, WireFrame};
use gpui::{
    AppContext as _, Application, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, PlatformInput, Point, ScrollDelta,
    ScrollWheelEvent, StreamingConfig, TouchPhase, px,
};
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

    let (app, frame_rx, resize_tx, input_tx) = Application::streaming(config);

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
        let input_tx_clone = input_tx.clone();
        let watch_rx = frame_watch_rx;
        tokio.spawn(async move {
            if let Err(e) =
                run_websocket_server(bind_addr, watch_rx, resize_tx_clone, input_tx_clone).await
            {
                error!("WebSocket server error: {e:#}");
            }
        });

        info!("zed_server listening on ws://{bind_addr}");
    });

    Ok(())
}

/// Reads DisplayTree frames from GPUI's smol channel, wraps each in a
/// WireFrame::Snapshot envelope, and publishes the protobuf-encoded bytes
/// to the watch channel.
async fn frame_bridge(
    frame_rx: smol::channel::Receiver<DisplayTree>,
    watch_tx: tokio_watch::Sender<Bytes>,
) {
    use prost::Message as _;
    loop {
        match frame_rx.recv().await {
            Ok(tree) => {
                let wire = WireFrame {
                    frame: Some(wire_frame::Frame::Snapshot(tree)),
                };
                let mut buf = Vec::with_capacity(wire.encoded_len());
                match wire.encode(&mut buf) {
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
    input_tx: smol::channel::Sender<PlatformInput>,
) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server bound to {addr}");

    loop {
        let (stream, peer) = listener.accept().await?;
        info!("new connection from {peer}");
        let rx = watch_rx.clone();
        let rtx = resize_tx.clone();
        let itx = input_tx.clone();
        tokio::spawn(handle_connection(stream, peer, rx, rtx, itx));
    }
}

/// Peek at the incoming bytes to determine if this is a WebSocket upgrade
/// or a plain HTTP request. Serve viewer.html for the latter.
async fn handle_connection(
    stream: TcpStream,
    peer: SocketAddr,
    watch_rx: tokio_watch::Receiver<Bytes>,
    resize_tx: smol::channel::Sender<(f32, f32, f32)>,
    input_tx: smol::channel::Sender<PlatformInput>,
) {
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;

    let mut buf = [0u8; 4096];
    let stream = match stream.peek(&mut buf).await {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buf[..n]);
            if request.contains("Upgrade: websocket") || request.contains("upgrade: websocket") {
                handle_client(stream, peer, watch_rx, resize_tx, input_tx).await;
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
    input_tx: smol::channel::Sender<PlatformInput>,
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
        use prost::Message as _;
        while let Ok(Some(msg)) = ws_rx.try_next().await {
            let data: Vec<u8> = match msg {
                Message::Binary(data) => data.to_vec(),
                Message::Text(text) => text.as_bytes().to_vec(),
                Message::Close(_) => break,
                _ => continue,
            };
            match WireFrame::decode(data.as_slice()) {
                Ok(frame) => match frame.frame {
                    Some(wire_frame::Frame::ViewportChanged(vc)) => {
                        let w = vc.width;
                        let h = vc.height;
                        let s = vc.scale_factor;
                        if w > 0.0 && h > 0.0 {
                            info!("{peer_recv} viewport: {w}x{h} @{s}x");
                            resize_tx.try_send((w, h, s)).ok();
                        }
                    }
                    Some(wire_frame::Frame::Action(ref action)) => {
                        let events = action_to_platform_input(action);
                        for event in events {
                            log::debug!("[server inject] {:?}", event);
                            input_tx.try_send(event).ok();
                        }
                    }
                    _ => {}
                },
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

/// Map DOM `KeyboardEvent.key` values to GPUI's lowercase key names.
/// Returns empty string for modifier-only keys that shouldn't generate events.
fn dom_key_to_gpui(dom_key: &str) -> String {
    match dom_key {
        // Modifier-only keys — no standalone event needed
        "Shift" | "Control" | "Alt" | "Meta" | "CapsLock" | "NumLock" | "ScrollLock" => {
            String::new()
        }
        // Arrow keys
        "ArrowLeft" => "left".into(),
        "ArrowRight" => "right".into(),
        "ArrowUp" => "up".into(),
        "ArrowDown" => "down".into(),
        // Navigation
        "Home" => "home".into(),
        "End" => "end".into(),
        "PageUp" => "pageup".into(),
        "PageDown" => "pagedown".into(),
        // Editing
        "Backspace" => "backspace".into(),
        "Delete" => "delete".into(),
        "Enter" => "enter".into(),
        "Tab" => "tab".into(),
        "Escape" => "escape".into(),
        "Insert" => "insert".into(),
        // Whitespace
        " " => "space".into(),
        // Function keys
        "F1" => "f1".into(),
        "F2" => "f2".into(),
        "F3" => "f3".into(),
        "F4" => "f4".into(),
        "F5" => "f5".into(),
        "F6" => "f6".into(),
        "F7" => "f7".into(),
        "F8" => "f8".into(),
        "F9" => "f9".into(),
        "F10" => "f10".into(),
        "F11" => "f11".into(),
        "F12" => "f12".into(),
        // Single characters — pass through as lowercase
        other => other.to_lowercase(),
    }
}

fn wire_modifiers_to_gpui(m: &gpui::display_tree::DisplayModifiers) -> Modifiers {
    Modifiers {
        control: m.control,
        alt: m.alt,
        shift: m.shift,
        platform: m.meta,
        function: false,
    }
}

fn wire_button_to_gpui(button: u32) -> MouseButton {
    match button {
        2 => MouseButton::Right,
        1 => MouseButton::Middle,
        _ => MouseButton::Left,
    }
}

fn action_to_platform_input(
    action: &gpui::display_tree::DisplayAction,
) -> Vec<PlatformInput> {
    let Some(kind) = action.action.as_ref().and_then(|k| k.action.as_ref()) else {
        return vec![];
    };

    match kind {
        display_action_kind::Action::Click(click) => {
            let pos = click
                .position
                .as_ref()
                .map(|p| Point::new(px(p.x), px(p.y)))
                .unwrap_or_default();
            let modifiers = click
                .modifiers
                .as_ref()
                .map(wire_modifiers_to_gpui)
                .unwrap_or_default();
            let button = wire_button_to_gpui(click.button);
            let click_count = click.click_count.max(1) as usize;
            vec![
                PlatformInput::MouseDown(MouseDownEvent {
                    button,
                    position: pos,
                    modifiers,
                    click_count,
                    first_mouse: false,
                }),
                PlatformInput::MouseUp(MouseUpEvent {
                    button,
                    position: pos,
                    modifiers,
                    click_count,
                }),
            ]
        }
        display_action_kind::Action::MouseDown(md) => {
            let pos = md
                .position
                .as_ref()
                .map(|p| Point::new(px(p.x), px(p.y)))
                .unwrap_or_default();
            let modifiers = md
                .modifiers
                .as_ref()
                .map(wire_modifiers_to_gpui)
                .unwrap_or_default();
            let button = wire_button_to_gpui(md.button);
            vec![PlatformInput::MouseDown(MouseDownEvent {
                button,
                position: pos,
                modifiers,
                click_count: 1,
                first_mouse: false,
            })]
        }
        display_action_kind::Action::MouseUp(mu) => {
            let pos = mu
                .position
                .as_ref()
                .map(|p| Point::new(px(p.x), px(p.y)))
                .unwrap_or_default();
            let modifiers = mu
                .modifiers
                .as_ref()
                .map(wire_modifiers_to_gpui)
                .unwrap_or_default();
            let button = wire_button_to_gpui(mu.button);
            vec![PlatformInput::MouseUp(MouseUpEvent {
                button,
                position: pos,
                modifiers,
                click_count: 1,
            })]
        }
        display_action_kind::Action::MouseMove(mm) => {
            let pos = mm
                .position
                .as_ref()
                .map(|p| Point::new(px(p.x), px(p.y)))
                .unwrap_or_default();
            let modifiers = mm
                .modifiers
                .as_ref()
                .map(wire_modifiers_to_gpui)
                .unwrap_or_default();
            vec![PlatformInput::MouseMove(MouseMoveEvent {
                position: pos,
                pressed_button: None,
                modifiers,
            })]
        }
        display_action_kind::Action::Scroll(scroll) => {
            let delta = scroll
                .delta
                .as_ref()
                .map(|d| Point::new(px(d.x), px(d.y)))
                .unwrap_or_default();
            let modifiers = scroll
                .modifiers
                .as_ref()
                .map(wire_modifiers_to_gpui)
                .unwrap_or_default();
            vec![PlatformInput::ScrollWheel(ScrollWheelEvent {
                position: Point::default(),
                delta: ScrollDelta::Pixels(delta),
                modifiers,
                touch_phase: TouchPhase::Moved,
            })]
        }
        display_action_kind::Action::KeyDown(kd) => {
            let wire_mods = kd.modifiers.as_ref();
            let modifiers = wire_mods
                .map(wire_modifiers_to_gpui)
                .unwrap_or_default();
            let gpui_key = dom_key_to_gpui(&kd.key);
            log::debug!(
                "[server keydown] dom_key={:?} -> gpui_key={:?} wire_mods={{ctrl={}, alt={}, shift={}, meta={}}} gpui_mods={{ctrl={}, alt={}, shift={}, platform={}}}",
                kd.key,
                gpui_key,
                wire_mods.map_or(false, |m| m.control),
                wire_mods.map_or(false, |m| m.alt),
                wire_mods.map_or(false, |m| m.shift),
                wire_mods.map_or(false, |m| m.meta),
                modifiers.control,
                modifiers.alt,
                modifiers.shift,
                modifiers.platform,
            );
            if gpui_key.is_empty() {
                log::debug!("[server keydown] skipping modifier-only key {:?}", kd.key);
                return vec![];
            }
            let key_char = if gpui_key.chars().count() == 1 && !modifiers.control && !modifiers.platform {
                Some(gpui_key.clone())
            } else {
                None
            };
            log::debug!(
                "[server keydown] keystroke: key={:?} key_char={:?} modifiers={:?}",
                gpui_key,
                key_char,
                modifiers,
            );
            vec![PlatformInput::KeyDown(KeyDownEvent {
                keystroke: Keystroke {
                    modifiers,
                    key: gpui_key,
                    key_char,
                },
                is_held: false,
                prefer_character_input: false,
            })]
        }
        display_action_kind::Action::KeyUp(ku) => {
            let modifiers = ku
                .modifiers
                .as_ref()
                .map(wire_modifiers_to_gpui)
                .unwrap_or_default();
            let gpui_key = dom_key_to_gpui(&ku.key);
            if gpui_key.is_empty() {
                return vec![];
            }
            vec![PlatformInput::KeyUp(KeyUpEvent {
                keystroke: Keystroke {
                    modifiers,
                    key: gpui_key,
                    key_char: None,
                },
            })]
        }
        display_action_kind::Action::Hover(_) | display_action_kind::Action::Resize(_) => {
            vec![]
        }
    }
}


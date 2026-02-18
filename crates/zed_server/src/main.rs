use anyhow::Result;
use bytes::Bytes;
use clap::Parser;
use futures::stream::StreamExt;
use futures::{SinkExt, TryStreamExt};
use gpui::display_tree::DisplayTree;
use gpui::{Application, StreamingConfig};
use log::{error, info};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

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

    let (app, frame_rx) = Application::streaming(config);

    let bind_addr = args.bind;
    let (broadcast_tx, _) = broadcast::channel::<Bytes>(16);

    app.run(move |cx| {
        settings::init(cx);
        gpui_tokio::init(cx);

        let tokio = gpui_tokio::Tokio::handle(cx).clone();
        let btx = broadcast_tx.clone();

        // Bridge: read DisplayTree frames from GPUI's smol channel,
        // serialize as WireFrame::Snapshot, broadcast to all WebSocket clients.
        tokio.spawn(frame_bridge(frame_rx, btx));

        // WebSocket server accepting browser clients.
        let btx2 = broadcast_tx.clone();
        tokio.spawn(async move {
            if let Err(e) = run_websocket_server(bind_addr, btx2).await {
                error!("WebSocket server error: {e:#}");
            }
        });

        info!("zed_server listening on ws://{bind_addr}");
    });

    Ok(())
}

/// Reads DisplayTree frames from GPUI's smol channel and broadcasts
/// serialized WireFrame::Snapshot bytes to all connected clients.
async fn frame_bridge(
    frame_rx: smol::channel::Receiver<DisplayTree>,
    broadcast_tx: broadcast::Sender<Bytes>,
) {
    use gpui::display_tree::WireFrame;

    loop {
        match frame_rx.recv().await {
            Ok(tree) => {
                let wire = WireFrame::Snapshot(tree);
                match wire.serialize() {
                    Ok(bytes) => {
                        let _ = broadcast_tx.send(Bytes::from(bytes));
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

/// Accepts WebSocket connections and forwards broadcast frames to each client.
async fn run_websocket_server(
    addr: SocketAddr,
    broadcast_tx: broadcast::Sender<Bytes>,
) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server bound to {addr}");

    loop {
        let (stream, peer) = listener.accept().await?;
        info!("new connection from {peer}");
        let rx = broadcast_tx.subscribe();
        tokio::spawn(handle_client(stream, peer, rx));
    }
}

/// Handles a single WebSocket client: sends frames, receives actions.
async fn handle_client(
    stream: TcpStream,
    peer: SocketAddr,
    mut frame_rx: broadcast::Receiver<Bytes>,
) {
    let ws = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("WebSocket handshake failed for {peer}: {e}");
            return;
        }
    };

    let (mut ws_tx, mut ws_rx) = ws.split();

    // Send frames to the browser client.
    let peer_send = peer;
    let send_task = tokio::spawn(async move {
        while let Ok(bytes) = frame_rx.recv().await {
            if ws_tx.send(Message::Binary(bytes.into())).await.is_err() {
                break;
            }
        }
        info!("{peer_send} send loop ended");
    });

    // Read incoming messages (DisplayActions from browser, future use).
    let peer_recv = peer;
    let recv_task = tokio::spawn(async move {
        while let Ok(Some(msg)) = ws_rx.try_next().await {
            match msg {
                Message::Binary(data) => {
                    // TODO: deserialize WireFrame::Action and inject via StreamingWindow
                    info!("{peer_recv} received {} bytes (action handling not yet implemented)", data.len());
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

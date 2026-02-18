//! WebSocket server for streaming GPUI scenes to browser clients.
//!
//! This module runs a lightweight WebSocket server using `async-tungstenite`
//! on the `smol` async runtime (which is already GPUI's background executor).
//! It accepts browser connections and handles bidirectional communication:
//!
//! - **Server -> Browser**: Serialized `FrameMessage` JSON after each draw call.
//! - **Browser -> Server**: `InputMessage` JSON containing mouse, keyboard,
//!   scroll, and resize events.
//!
//! The server is designed to integrate with the `WebStreamingClient` and its
//! calloop event loop. Frame data flows from the main thread (where GPUI's
//! draw cycle runs) to connected WebSocket clients via a broadcast channel.
//! Input events flow back from the WebSocket read loop to the main thread
//! via calloop channel, where they are dispatched through GPUI's input system.
//!
//! Architecture:
//!
//! ```text
//!  ┌──────────────┐    broadcast     ┌──────────────────┐    WebSocket
//!  │  GPUI draw   │───────────────>  │  Server task     │ ──────────> Browser
//!  │  (calloop)   │                  │  (smol executor) │
//!  │              │  <─────────────  │                  │ <────────── Browser
//!  └──────────────┘  calloop channel └──────────────────┘    WebSocket
//! ```

#[cfg(feature = "web-streaming")]
mod inner {
    use std::net::SocketAddr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU64, Ordering};

    use anyhow::{Context as _, Result};
    use async_tungstenite::tungstenite::Message;
    use futures::channel::mpsc;
    use futures::{SinkExt, StreamExt};
    use parking_lot::Mutex;
    use smol::net::{TcpListener, TcpStream};

    use super::super::scene_message::FrameMessage;

    /// A handle to the running WebSocket server. Used by the streaming platform
    /// to broadcast frames and receive input events.
    pub struct WebStreamingServer {
        /// Send serialized frame data to all connected clients.
        frame_tx: Arc<Mutex<Vec<mpsc::UnboundedSender<Arc<Vec<u8>>>>>>,

        /// Receive input messages from browser clients.
        /// Each message is a raw JSON string from the WebSocket.
        input_rx: mpsc::UnboundedReceiver<ClientInput>,

        /// Sender side kept so we can clone it for new client tasks.
        input_tx: mpsc::UnboundedSender<ClientInput>,

        /// Monotonically increasing frame counter.
        frame_counter: AtomicU64,

        /// The address the server is listening on.
        listen_addr: SocketAddr,
    }

    /// An input event received from a browser client.
    #[derive(Debug)]
    pub struct ClientInput {
        /// Raw JSON string of the input message.
        pub json: String,
    }

    impl WebStreamingServer {
        /// Start the WebSocket server listening on the given address.
        ///
        /// This spawns the accept loop on the smol background executor.
        /// The server is immediately ready to accept connections after this
        /// returns.
        pub async fn start(addr: &str) -> Result<Self> {
            let listener = TcpListener::bind(addr)
                .await
                .context("Failed to bind WebSocket server")?;

            let listen_addr = listener.local_addr()?;
            log::info!("Web streaming server listening on ws://{}", listen_addr);

            let clients: Arc<Mutex<Vec<mpsc::UnboundedSender<Arc<Vec<u8>>>>>> =
                Arc::new(Mutex::new(Vec::new()));
            let (input_tx, input_rx) = mpsc::unbounded();

            let accept_clients = clients.clone();
            let accept_input_tx = input_tx.clone();

            // Spawn the accept loop on the smol executor.
            smol::spawn(async move {
                loop {
                    match listener.accept().await {
                        Ok((stream, peer_addr)) => {
                            log::info!("Web streaming client connected: {}", peer_addr);
                            Self::handle_connection(
                                stream,
                                peer_addr,
                                accept_clients.clone(),
                                accept_input_tx.clone(),
                            );
                        }
                        Err(err) => {
                            log::error!("Failed to accept WebSocket connection: {}", err);
                        }
                    }
                }
            })
            .detach();

            Ok(Self {
                frame_tx: clients,
                input_rx,
                input_tx,
                frame_counter: AtomicU64::new(0),
                listen_addr,
            })
        }

        /// Broadcast a JSON frame to all connected browser clients.
        pub fn broadcast_frame(&self, frame: &FrameMessage) {
            let json = match serde_json::to_string(frame) {
                Ok(json) => json.into_bytes(),
                Err(err) => {
                    log::error!("Failed to serialize frame: {}", err);
                    return;
                }
            };
            self.broadcast_binary(&json);
        }

        /// Broadcast raw binary data to all connected browser clients.
        ///
        /// Sends the bytes as a binary WebSocket message. Disconnected
        /// clients are pruned automatically.
        pub fn broadcast_binary(&self, data: &[u8]) {
            let data = Arc::new(data.to_vec());

            self.frame_counter.fetch_add(1, Ordering::Relaxed);

            let mut clients = self.frame_tx.lock();
            clients.retain(|tx| tx.unbounded_send(data.clone()).is_ok());
        }

        /// Try to receive the next input event from any connected browser client.
        ///
        /// Returns `None` if no input is available. This is non-blocking.
        pub fn try_recv_input(&mut self) -> Option<ClientInput> {
            self.input_rx.try_next().ok().flatten()
        }

        /// The address the server is listening on.
        pub fn listen_addr(&self) -> SocketAddr {
            self.listen_addr
        }

        /// Current frame counter (number of frames broadcast).
        pub fn frame_count(&self) -> u64 {
            self.frame_counter.load(Ordering::Relaxed)
        }

        /// Number of currently connected clients.
        pub fn client_count(&self) -> usize {
            self.frame_tx.lock().len()
        }

        /// Handle a single incoming TCP connection by upgrading it to WebSocket
        /// and spawning read/write tasks.
        fn handle_connection(
            stream: TcpStream,
            peer_addr: SocketAddr,
            clients: Arc<Mutex<Vec<mpsc::UnboundedSender<Arc<Vec<u8>>>>>>,
            input_tx: mpsc::UnboundedSender<ClientInput>,
        ) {
            smol::spawn(async move {
                let ws_stream = match async_tungstenite::accept_async(stream).await {
                    Ok(ws) => ws,
                    Err(err) => {
                        log::error!("WebSocket handshake failed for {}: {}", peer_addr, err);
                        return;
                    }
                };

                let (write, mut read) = ws_stream.split();

                // Create a per-client channel for outgoing frames.
                let (client_tx, client_rx) = mpsc::unbounded::<Arc<Vec<u8>>>();

                // Register this client's sender.
                clients.lock().push(client_tx);

                // Spawn the write task: forward frames from the channel to the WebSocket.
                let write_handle = smol::spawn(async move {
                    let mut sink = write;
                    let mut rx = client_rx;
                    while let Some(data) = rx.next().await {
                        let msg = Message::Binary((*data).clone().into());
                        if futures::SinkExt::send(&mut sink, msg).await.is_err() {
                            log::warn!("Failed to send frame to {}", peer_addr);
                            break;
                        }
                    }
                });

                // Read loop: receive input events from the browser and forward them.
                while let Some(msg_result) = read.next().await {
                    match msg_result {
                        Ok(Message::Text(text)) => {
                            if input_tx
                                .unbounded_send(ClientInput {
                                    json: text.to_string(),
                                })
                                .is_err()
                            {
                                // Main thread receiver dropped -- shut down.
                                break;
                            }
                        }
                        Ok(Message::Binary(data)) => {
                            // Try to interpret binary messages as UTF-8 JSON.
                            if let Ok(text) = String::from_utf8(data.to_vec()) {
                                let _ = input_tx.unbounded_send(ClientInput { json: text });
                            }
                        }
                        Ok(Message::Close(_)) => {
                            log::info!("Web streaming client disconnected: {}", peer_addr);
                            break;
                        }
                        Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => {
                            // Handled automatically by tungstenite.
                        }
                        Ok(Message::Frame(_)) => {
                            // Raw frames -- ignore.
                        }
                        Err(err) => {
                            log::warn!(
                                "WebSocket error from {}: {}. Disconnecting.",
                                peer_addr,
                                err
                            );
                            break;
                        }
                    }
                }

                // Cancel the write task when the read loop exits.
                write_handle.cancel().await;
                log::info!("Web streaming client fully disconnected: {}", peer_addr);
            })
            .detach();
        }
    }
}

// Re-export the inner module's types when the feature is enabled.
#[cfg(feature = "web-streaming")]
pub use inner::*;

// When the feature is not enabled, provide stub types so the rest of the
// module can reference them without cfg gates everywhere.
#[cfg(not(feature = "web-streaming"))]
mod stub {
    use anyhow::Result;
    use std::net::SocketAddr;

    use super::super::scene_message::FrameMessage;

    pub struct ClientInput {
        pub json: String,
    }

    pub struct WebStreamingServer;

    impl WebStreamingServer {
        pub async fn start(_addr: &str) -> Result<Self> {
            anyhow::bail!("Web streaming support requires the `web-streaming` feature flag")
        }

        pub fn broadcast_frame(&self, _frame: &FrameMessage) {}

        pub fn try_recv_input(&mut self) -> Option<ClientInput> {
            None
        }

        pub fn listen_addr(&self) -> SocketAddr {
            SocketAddr::from(([0, 0, 0, 0], 0))
        }

        pub fn frame_count(&self) -> u64 {
            0
        }

        pub fn client_count(&self) -> usize {
            0
        }
    }
}

#[cfg(not(feature = "web-streaming"))]
pub use stub::*;

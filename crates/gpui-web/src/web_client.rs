use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use futures::{
    FutureExt as _,
    channel::oneshot,
    future::BoxFuture,
};
use parking_lot::Mutex;
use prost::Message as ProstMessage;
use proto::{
    Envelope, EnvelopedMessage, Instant, PeerId,
    build_typed_envelope,
};
use rpc::{ProtoClient, ProtoMessageHandlerSet};
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering::SeqCst},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{BinaryType, MessageEvent, WebSocket};

type ResponseChannels = Mutex<HashMap<u32, oneshot::Sender<Envelope>>>;

pub struct WebProtoClient {
    websocket: WebSocket,
    next_message_id: AtomicU32,
    response_channels: Arc<ResponseChannels>,
    message_handlers: Mutex<ProtoMessageHandlerSet>,
}

// web_sys::WebSocket is !Send + !Sync on WASM, but WASM is single-threaded.
// ProtoClient requires Send + Sync bounds for cross-platform compatibility.
unsafe impl Send for WebProtoClient {}
unsafe impl Sync for WebProtoClient {}

impl WebProtoClient {
    pub fn new(url: &str, cx: &gpui::AsyncApp) -> Result<Arc<Self>> {
        let websocket = WebSocket::new(url)
            .map_err(|error| anyhow!("failed to create WebSocket: {error:?}"))?;
        websocket.set_binary_type(BinaryType::Arraybuffer);

        let response_channels = Arc::new(ResponseChannels::default());
        let message_handlers = Mutex::new(ProtoMessageHandlerSet::default());

        let client = Arc::new(Self {
            websocket,
            next_message_id: AtomicU32::new(0),
            response_channels,
            message_handlers,
        });

        Self::install_message_handler(&client, cx.clone());

        Ok(client)
    }

    fn install_message_handler(client: &Arc<Self>, cx: gpui::AsyncApp) {
        let weak = Arc::downgrade(client);

        let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
            let Some(this) = weak.upgrade() else { return };

            let Ok(buffer) = event.data().dyn_into::<js_sys::ArrayBuffer>() else {
                return;
            };
            let array = js_sys::Uint8Array::new(&buffer);
            let data = array.to_vec();

            let envelope = match Envelope::decode(data.as_slice()) {
                Ok(envelope) => envelope,
                Err(error) => {
                    web_sys::console::error_1(
                        &format!("WebProtoClient: decode error: {error:?}").into(),
                    );
                    return;
                }
            };

            // Handle responses to our requests
            if let Some(request_id) = envelope.responding_to {
                let sender = this.response_channels.lock().remove(&request_id);
                if let Some(sender) = sender {
                    sender.send(envelope).ok();
                }
                return;
            }

            // Handle RemoteStarted handshake
            if let Some(proto::envelope::Payload::RemoteStarted(_)) = &envelope.payload {
                let mut ack = proto::Ack {}.into_envelope(0, Some(envelope.id), None);
                ack.id = this.next_message_id.fetch_add(1, SeqCst);
                this.send_envelope(ack).ok();
                return;
            }

            // Handle FlushBufferedMessages
            if let Some(proto::envelope::Payload::FlushBufferedMessages(_)) = &envelope.payload {
                let mut ack = proto::Ack {}.into_envelope(0, Some(envelope.id), None);
                ack.id = this.next_message_id.fetch_add(1, SeqCst);
                this.send_envelope(ack).ok();
                return;
            }

            let peer_id = PeerId { owner_id: 0, id: 0 };
            if let Some(typed_envelope) =
                build_typed_envelope(peer_id, Instant::now(), envelope)
            {
                let type_name = typed_envelope.payload_type_name();
                if let Some(future) = ProtoMessageHandlerSet::handle_message(
                    &this.message_handlers,
                    typed_envelope,
                    this.clone().into(),
                    cx.clone(),
                ) {
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Err(error) = future.await {
                            web_sys::console::error_1(
                                &format!(
                                    "WebProtoClient: error handling {type_name}: {error:#}"
                                )
                                .into(),
                            );
                        }
                    });
                } else {
                    web_sys::console::log_1(
                        &format!("WebProtoClient: no handler for {type_name}").into(),
                    );
                }
            }
        });

        client
            .websocket
            .set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();
    }

    fn send_envelope(&self, envelope: Envelope) -> Result<()> {
        let mut buf = Vec::with_capacity(envelope.encoded_len());
        envelope
            .encode(&mut buf)
            .context("failed to encode envelope")?;
        self.websocket
            .send_with_u8_array(&buf)
            .map_err(|error| anyhow!("WebSocket send failed: {error:?}"))?;
        Ok(())
    }

    /// Send the initial `RemoteStarted` handshake to the server.
    pub fn send_remote_started(&self) -> Result<()> {
        let mut envelope = proto::RemoteStarted {}.into_envelope(0, None, None);
        envelope.id = self.next_message_id.fetch_add(1, SeqCst);
        self.send_envelope(envelope)
    }
}

impl ProtoClient for WebProtoClient {
    fn request(
        &self,
        mut envelope: Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<Envelope>> {
        envelope.id = self.next_message_id.fetch_add(1, SeqCst);
        let (tx, rx) = oneshot::channel();
        self.response_channels.lock().insert(envelope.id, tx);

        let send_result = self.send_envelope(envelope);
        async move {
            send_result?;
            let response = rx.await.context("connection lost")?;
            if let Some(proto::envelope::Payload::Error(error)) = &response.payload {
                return Err(anyhow!(
                    "RPC error for {request_type}: {}",
                    error.message
                ));
            }
            Ok(response)
        }
        .boxed()
    }

    fn send(&self, mut envelope: Envelope, _message_type: &'static str) -> Result<()> {
        envelope.id = self.next_message_id.fetch_add(1, SeqCst);
        self.send_envelope(envelope)
    }

    fn send_response(&self, mut envelope: Envelope, _message_type: &'static str) -> Result<()> {
        envelope.id = self.next_message_id.fetch_add(1, SeqCst);
        self.send_envelope(envelope)
    }

    fn message_handler_set(&self) -> &Mutex<ProtoMessageHandlerSet> {
        &self.message_handlers
    }

    fn is_via_collab(&self) -> bool {
        false
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }
}

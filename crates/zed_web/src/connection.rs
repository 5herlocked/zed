use futures::channel::mpsc;
use gpui::display_tree::WireFrame;
use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::prelude::*;
use web_sys::{BinaryType, CloseEvent, ErrorEvent, MessageEvent, WebSocket};

/// WebSocket connection to the Zed streaming server.
///
/// Incoming binary messages are deserialized as WireFrames and pushed into the
/// `frame_tx` channel. Outgoing WireFrames (actions, viewport changes, pongs)
/// are sent directly via `send()`.
pub struct Connection {
    ws: WebSocket,
    // Store closures so they aren't dropped (dropping a Closure cancels the callback).
    _on_message: Closure<dyn FnMut(MessageEvent)>,
    _on_error: Closure<dyn FnMut(ErrorEvent)>,
    _on_close: Closure<dyn FnMut(CloseEvent)>,
}

impl Connection {
    /// Open a WebSocket to `url` and begin routing incoming frames to `frame_tx`.
    pub fn connect(
        url: &str,
        frame_tx: mpsc::UnboundedSender<WireFrame>,
    ) -> Result<Self, JsValue> {
        let ws = WebSocket::new(url)?;
        ws.set_binary_type(BinaryType::Arraybuffer);

        let on_message = {
            let tx = frame_tx.clone();
            Closure::wrap(Box::new(move |event: MessageEvent| {
                let data = event.data();
                let buffer = match data.dyn_into::<ArrayBuffer>() {
                    Ok(buf) => buf,
                    Err(_) => {
                        log::warn!("received non-binary WebSocket message, ignoring");
                        return;
                    }
                };
                let bytes = Uint8Array::new(&buffer).to_vec();
                match WireFrame::deserialize(&bytes) {
                    Ok(frame) => {
                        if tx.unbounded_send(frame).is_err() {
                            log::error!("frame channel closed, dropping message");
                        }
                    }
                    Err(e) => log::error!("failed to deserialize WireFrame: {:?}", e),
                }
            }) as Box<dyn FnMut(MessageEvent)>)
        };
        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));

        let on_error = Closure::wrap(Box::new(move |e: ErrorEvent| {
            log::error!("WebSocket error: {:?}", e.message());
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));

        let on_close = Closure::wrap(Box::new(move |e: CloseEvent| {
            log::info!(
                "WebSocket closed: code={} reason={}",
                e.code(),
                e.reason()
            );
        }) as Box<dyn FnMut(CloseEvent)>);
        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));

        log::info!("WebSocket connecting to {}", url);

        Ok(Self {
            ws,
            _on_message: on_message,
            _on_error: on_error,
            _on_close: on_close,
        })
    }

    /// Send a WireFrame to the server (serialized as postcard binary).
    pub fn send(&self, frame: &WireFrame) -> Result<(), JsValue> {
        let bytes = frame.serialize().map_err(|e| {
            JsValue::from_str(&format!("serialization error: {:?}", e))
        })?;
        let array = Uint8Array::from(&bytes[..]);
        self.ws.send_with_array_buffer_view(&array)
    }
}

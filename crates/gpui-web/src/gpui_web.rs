use prost::Message as ProstMessage;
use proto::{Envelope, envelope::Payload};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{BinaryType, MessageEvent, WebSocket};

fn payload_type_name(payload: &Payload) -> &'static str {
    match payload {
        Payload::Hello(_) => "Hello",
        Payload::Ack(_) => "Ack",
        Payload::Error(_) => "Error",
        Payload::Ping(_) => "Ping",
        Payload::Test(_) => "Test",
        Payload::EndStream(_) => "EndStream",
        Payload::UpdateWorktree(_) => "UpdateWorktree",
        Payload::UpdateWorktreeSettings(_) => "UpdateWorktreeSettings",
        Payload::CreateBufferForPeer(_) => "CreateBufferForPeer",
        Payload::UpdateBuffer(_) => "UpdateBuffer",
        Payload::OpenBufferByPath(_) => "OpenBufferByPath",
        Payload::OpenBufferResponse(_) => "OpenBufferResponse",
        _ => "Other",
    }
}

/// Initialize the WASM module and connect to the WebSocket server.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    log::info!("gpui-web: WASM module initialized");
}

/// Connect to a WebSocket server and log proto messages to the browser console.
#[wasm_bindgen]
pub fn connect(url: &str) -> Result<(), JsValue> {
    let ws = WebSocket::new(url)?;
    ws.set_binary_type(BinaryType::Arraybuffer);

    let onopen = Closure::<dyn FnMut()>::new(move || {
        web_sys::console::log_1(&"gpui-web: WebSocket connected".into());
    });
    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();

    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        if let Ok(buffer) = event.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&buffer);
            let data = array.to_vec();

            match Envelope::decode(data.as_slice()) {
                Ok(envelope) => {
                    let type_name = envelope
                        .payload
                        .as_ref()
                        .map(payload_type_name)
                        .unwrap_or("Empty");

                    let msg = format!(
                        "gpui-web: [{}] id={} responding_to={:?} type={}",
                        data.len(),
                        envelope.id,
                        envelope.responding_to,
                        type_name,
                    );
                    web_sys::console::log_1(&msg.into());
                }
                Err(error) => {
                    let msg = format!("gpui-web: failed to decode envelope: {error:?}");
                    web_sys::console::error_1(&msg.into());
                }
            }
        }
    });
    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::new(move |event: web_sys::ErrorEvent| {
        let msg = format!("gpui-web: WebSocket error: {:?}", event.message());
        web_sys::console::error_1(&msg.into());
    });
    ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onerror.forget();

    let onclose = Closure::<dyn FnMut(web_sys::CloseEvent)>::new(move |event: web_sys::CloseEvent| {
        let msg = format!(
            "gpui-web: WebSocket closed: code={} reason={}",
            event.code(),
            event.reason()
        );
        web_sys::console::log_1(&msg.into());
    });
    ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
    onclose.forget();

    Ok(())
}

mod file_tree_view;
mod web_buffer_store;
mod web_client;
mod web_worktree_store;

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
        Payload::UpdateDiagnosticSummary(_) => "UpdateDiagnosticSummary",
        Payload::OpenBufferByPath(_) => "OpenBufferByPath",
        Payload::OpenBufferResponse(_) => "OpenBufferResponse",
        Payload::UpdateProject(_) => "UpdateProject",
        Payload::CreateProjectEntry(_) => "CreateProjectEntry",
        Payload::RenameProjectEntry(_) => "RenameProjectEntry",
        Payload::DeleteProjectEntry(_) => "DeleteProjectEntry",
        _ => "Other",
    }
}

fn payload_summary(payload: &Payload) -> String {
    match payload {
        Payload::Hello(hello) => format!("peer_id: {:?}", hello.peer_id),
        Payload::Ack(_) => String::new(),
        Payload::Error(error) => format!("message: {}", error.message),
        Payload::UpdateWorktree(update) => format!(
            "project_id: {}, worktree_id: {}, updated: {}, removed: {}",
            update.project_id,
            update.worktree_id,
            update.updated_entries.len(),
            update.removed_entries.len(),
        ),
        Payload::CreateBufferForPeer(create) => format!(
            "project_id: {}, peer_id: {:?}",
            create.project_id, create.peer_id,
        ),
        Payload::UpdateBuffer(update) => format!(
            "project_id: {}, buffer_id: {}",
            update.project_id, update.buffer_id,
        ),
        Payload::UpdateDiagnosticSummary(update) => format!(
            "project_id: {}, worktree_id: {}",
            update.project_id, update.worktree_id,
        ),
        _ => String::new(),
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Connect to a WebSocket server and log proto messages to the browser console.
#[wasm_bindgen]
pub fn connect(url: &str) -> Result<(), JsValue> {
    connect_with_callback(url, &JsValue::UNDEFINED)
}

/// Connect to a WebSocket server and call `on_message(json_string)` for each
/// decoded proto Envelope. If `on_message` is undefined, logs to console.
#[wasm_bindgen]
pub fn connect_with_callback(url: &str, on_message: &JsValue) -> Result<(), JsValue> {
    let ws = WebSocket::new(url)?;
    ws.set_binary_type(BinaryType::Arraybuffer);

    let callback: Option<js_sys::Function> = if on_message.is_function() {
        Some(on_message.clone().unchecked_into())
    } else {
        None
    };

    let onopen = Closure::<dyn FnMut()>::new(move || {
        web_sys::console::log_1(&"gpui-web: WebSocket connected".into());
    });
    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();

    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new({
        let callback = callback.clone();
        move |event: MessageEvent| {
            if let Ok(buffer) = event.data().dyn_into::<js_sys::ArrayBuffer>() {
                let array = js_sys::Uint8Array::new(&buffer);
                let data = array.to_vec();
                let byte_len = data.len();

                match Envelope::decode(data.as_slice()) {
                    Ok(envelope) => {
                        let type_name = envelope
                            .payload
                            .as_ref()
                            .map(payload_type_name)
                            .unwrap_or("Empty");

                        let summary = envelope
                            .payload
                            .as_ref()
                            .map(payload_summary)
                            .unwrap_or_default();

                        if let Some(ref cb) = callback {
                            let json = format!(
                                r#"{{"id":{},"responding_to":{},"type":"{}","bytes":{},"summary":"{}"}}"#,
                                envelope.id,
                                envelope.responding_to.map_or("null".to_string(), |v| v.to_string()),
                                type_name,
                                byte_len,
                                summary.replace('"', r#"\""#),
                            );
                            cb.call1(&JsValue::NULL, &JsValue::from_str(&json)).ok();
                        } else {
                            let msg = format!(
                                "[{}B] id={} type={} {}",
                                byte_len, envelope.id, type_name, summary,
                            );
                            web_sys::console::log_1(&msg.into());
                        }
                    }
                    Err(error) => {
                        let msg = format!("gpui-web: decode error: {error:?}");
                        web_sys::console::error_1(&msg.into());
                    }
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

    let onclose_cb = callback;
    let onclose = Closure::<dyn FnMut(web_sys::CloseEvent)>::new(move |event: web_sys::CloseEvent| {
        let msg = format!(
            "gpui-web: WebSocket closed: code={} reason={}",
            event.code(),
            event.reason()
        );
        web_sys::console::log_1(&msg.into());
        if let Some(ref cb) = onclose_cb {
            let json = r#"{"type":"_Disconnected","id":0,"bytes":0,"summary":"connection closed"}"#;
            cb.call1(&JsValue::NULL, &JsValue::from_str(json)).ok();
        }
    });
    ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
    onclose.forget();

    Ok(())
}

/// Launch the GPUI-based file tree view connected to a Zed remote server.
///
/// This creates a GPUI application, connects to the server via WebSocket,
/// registers proto message handlers, and renders a file tree from the
/// server's worktree state.
#[wasm_bindgen]
pub fn launch_file_tree(url: &str) -> Result<(), JsValue> {
    use crate::file_tree_view::FileTreeView;
    use crate::web_buffer_store::WebBufferStore;
    use crate::web_client::WebProtoClient;
    use crate::web_worktree_store::WebWorktreeStore;
    use gpui::AppContext as _;
    use rpc::AnyProtoClient;

    let url = url.to_string();

    let app = gpui::Application::new();
    app.run(move |cx| {
        let async_cx = cx.to_async();

        let client = match WebProtoClient::new(&url, &async_cx) {
            Ok(client) => client,
            Err(error) => {
                web_sys::console::error_1(
                    &format!("Failed to create WebProtoClient: {error:#}").into(),
                );
                return;
            }
        };

        let proto_client: AnyProtoClient = client.clone().into();

        let worktree_store = cx.new(|_cx| WebWorktreeStore::new(0));
        let buffer_store = cx.new(|_cx| WebBufferStore::new(0));

        WebWorktreeStore::register_message_handlers(&proto_client, &worktree_store);
        WebBufferStore::register_message_handlers(&proto_client, &buffer_store);

        proto_client.subscribe_to_entity::<WebWorktreeStore>(0, &worktree_store);
        proto_client.subscribe_to_entity::<WebBufferStore>(0, &buffer_store);

        if let Err(error) = client.send_remote_started() {
            web_sys::console::error_1(
                &format!("Failed to send RemoteStarted: {error:#}").into(),
            );
        }

        web_sys::console::log_1(&"gpui-web: File tree view launched".into());
    });

    Ok(())
}

use futures::channel::mpsc;
use gpui::display_tree::{
    wire_frame, ClickAction, DisplayAction, DisplayActionKind, DisplayModifiers, DisplayNodeId,
    HoverAction, KeyDownAction, KeyUpAction, MouseDownAction, MouseUpAction, Point, ScrollAction,
    ViewportChanged, WireFrame,
};
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

    /// Send a WireFrame to the server (serialized as protobuf binary).
    pub fn send(&self, frame: &WireFrame) -> Result<(), JsValue> {
        let bytes = frame
            .serialize()
            .map_err(|e| JsValue::from_str(&format!("serialization error: {:?}", e)))?;
        let array = Uint8Array::from(&bytes[..]);
        self.ws.send_with_array_buffer_view(&array)
    }

    /// Send a click action for a specific display node.
    pub fn send_click(
        &self,
        node_id: u64,
        element_id: Option<String>,
        x: f32,
        y: f32,
        button: u32,
        click_count: u32,
        modifiers: DisplayModifiers,
    ) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::Action(DisplayAction {
                node_id: Some(DisplayNodeId { id: node_id }),
                element_id,
                action: Some(DisplayActionKind {
                    action: Some(gpui::display_tree::display_action_kind::Action::Click(
                        ClickAction {
                            position: Some(Point { x, y }),
                            button,
                            click_count,
                            modifiers: Some(modifiers),
                        },
                    )),
                }),
            })),
        })
    }

    /// Send a mouse down action.
    pub fn send_mouse_down(
        &self,
        node_id: u64,
        element_id: Option<String>,
        x: f32,
        y: f32,
        button: u32,
        modifiers: DisplayModifiers,
    ) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::Action(DisplayAction {
                node_id: Some(DisplayNodeId { id: node_id }),
                element_id,
                action: Some(DisplayActionKind {
                    action: Some(
                        gpui::display_tree::display_action_kind::Action::MouseDown(
                            MouseDownAction {
                                position: Some(Point { x, y }),
                                button,
                                modifiers: Some(modifiers),
                            },
                        ),
                    ),
                }),
            })),
        })
    }

    /// Send a mouse up action.
    pub fn send_mouse_up(
        &self,
        node_id: u64,
        element_id: Option<String>,
        x: f32,
        y: f32,
        button: u32,
        modifiers: DisplayModifiers,
    ) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::Action(DisplayAction {
                node_id: Some(DisplayNodeId { id: node_id }),
                element_id,
                action: Some(DisplayActionKind {
                    action: Some(gpui::display_tree::display_action_kind::Action::MouseUp(
                        MouseUpAction {
                            position: Some(Point { x, y }),
                            button,
                            modifiers: Some(modifiers),
                        },
                    )),
                }),
            })),
        })
    }

    /// Send a hover action.
    pub fn send_hover(
        &self,
        node_id: u64,
        element_id: Option<String>,
        entered: bool,
    ) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::Action(DisplayAction {
                node_id: Some(DisplayNodeId { id: node_id }),
                element_id,
                action: Some(DisplayActionKind {
                    action: Some(gpui::display_tree::display_action_kind::Action::Hover(
                        HoverAction { entered },
                    )),
                }),
            })),
        })
    }

    /// Send a scroll action.
    pub fn send_scroll(
        &self,
        node_id: u64,
        element_id: Option<String>,
        delta_x: f32,
        delta_y: f32,
        modifiers: DisplayModifiers,
    ) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::Action(DisplayAction {
                node_id: Some(DisplayNodeId { id: node_id }),
                element_id,
                action: Some(DisplayActionKind {
                    action: Some(gpui::display_tree::display_action_kind::Action::Scroll(
                        ScrollAction {
                            delta: Some(Point {
                                x: delta_x,
                                y: delta_y,
                            }),
                            modifiers: Some(modifiers),
                        },
                    )),
                }),
            })),
        })
    }

    /// Send a key down action (not targeted at a specific node).
    pub fn send_key_down(
        &self,
        key: String,
        modifiers: DisplayModifiers,
    ) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::Action(DisplayAction {
                node_id: None,
                element_id: None,
                action: Some(DisplayActionKind {
                    action: Some(gpui::display_tree::display_action_kind::Action::KeyDown(
                        KeyDownAction {
                            key,
                            modifiers: Some(modifiers),
                        },
                    )),
                }),
            })),
        })
    }

    /// Send a key up action (not targeted at a specific node).
    pub fn send_key_up(&self, key: String, modifiers: DisplayModifiers) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::Action(DisplayAction {
                node_id: None,
                element_id: None,
                action: Some(DisplayActionKind {
                    action: Some(gpui::display_tree::display_action_kind::Action::KeyUp(
                        KeyUpAction {
                            key,
                            modifiers: Some(modifiers),
                        },
                    )),
                }),
            })),
        })
    }

    /// Notify the server that the browser viewport has changed.
    pub fn send_viewport_changed(
        &self,
        width: f32,
        height: f32,
        scale_factor: f32,
    ) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::ViewportChanged(ViewportChanged {
                width,
                height,
                scale_factor,
            })),
        })
    }

    /// Request a full snapshot from the server.
    pub fn request_snapshot(&self) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::RequestSnapshot(
                gpui::display_tree::Empty {},
            )),
        })
    }

    /// Send a pong response.
    pub fn send_pong(&self, seq: u64) -> Result<(), JsValue> {
        self.send(&WireFrame {
            frame: Some(wire_frame::Frame::Pong(seq)),
        })
    }
}

/// Convert a browser `web_sys::MouseEvent` into `DisplayModifiers`.
pub fn modifiers_from_mouse_event(event: &web_sys::MouseEvent) -> DisplayModifiers {
    DisplayModifiers {
        control: event.ctrl_key(),
        alt: event.alt_key(),
        shift: event.shift_key(),
        meta: event.meta_key(),
    }
}

/// Convert a browser `web_sys::KeyboardEvent` into `DisplayModifiers`.
pub fn modifiers_from_keyboard_event(event: &web_sys::KeyboardEvent) -> DisplayModifiers {
    DisplayModifiers {
        control: event.ctrl_key(),
        alt: event.alt_key(),
        shift: event.shift_key(),
        meta: event.meta_key(),
    }
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod connection;
mod remote_view;

pub use connection::Connection;
use remote_view::RemoteView;

use gpui::display_tree::WireFrame;
use std::rc::Rc;

/// Called automatically when the WASM module loads.
/// Sets up panic reporting and console logging so errors and log::info!()
/// calls show up in the browser's developer console.
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).ok();
    log::info!("zed_web: module loaded");
}

/// Boot the Zed web client.
///
/// Connects to a streaming server at `ws_url` (e.g. "ws://localhost:8080/stream"),
/// opens a GPUI window with a RemoteView, and starts piping DisplayTree frames
/// from the server into the view. The browser renders the element tree locally
/// using GPUI's web platform.
#[wasm_bindgen]
pub fn launch(ws_url: &str) -> Result<(), JsValue> {
    use futures::StreamExt;
    use gpui::{AppContext, Application, WindowOptions};

    let url = ws_url.to_string();

    // Use web_sys console directly since log crate may not flush before panic.
    fn console_log(msg: &str) {
        web_sys::console::log_1(&JsValue::from_str(msg));
    }

    console_log("zed_web: creating Application");
    let app = Application::new();

    console_log("zed_web: Application created, calling app.run()");
    app.run(move |cx| {
        console_log("zed_web: inside app.run callback");

        let (frame_tx, mut frame_rx) = futures::channel::mpsc::unbounded::<WireFrame>();

        console_log(&format!("zed_web: connecting WebSocket to {}", url));
        let connection = Rc::new(
            Connection::connect(&url, frame_tx).expect("failed to connect WebSocket"),
        );

        // Send initial viewport size to server.
        if let Some(browser_window) = web_sys::window() {
            let width = browser_window
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1280.0) as f32;
            let height = browser_window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(720.0) as f32;
            let scale = browser_window.device_pixel_ratio() as f32;
            connection
                .send_viewport_changed(width, height, scale)
                .ok();
        }

        setup_resize_listener(&connection);
        setup_keyboard_listener(&connection);

        console_log("zed_web: opening GPUI window");
        let conn = connection.clone();
        let window = cx
            .open_window(WindowOptions::default(), |_window, cx| {
                console_log("zed_web: building root view");
                cx.new(|_cx| RemoteView::new(conn))
            })
            .expect("failed to open window");

        console_log("zed_web: window opened, spawning frame loop");
        cx.spawn(async move |cx| {
            while let Some(frame) = frame_rx.next().await {
                window
                    .update(cx, |view: &mut RemoteView, _window, cx| {
                        view.apply_frame(frame);
                        cx.notify();
                    })
                    .ok();
            }
        })
        .detach();

        console_log("zed_web: launched, connected to server");
    });

    Ok(())
}

/// Attach a `resize` event listener on the browser window that forwards
/// viewport size changes to the streaming server.
fn setup_resize_listener(connection: &Rc<Connection>) {
    let conn = connection.clone();
    let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
        if let Some(browser_window) = web_sys::window() {
            let width = browser_window
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1280.0) as f32;
            let height = browser_window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(720.0) as f32;
            let scale = browser_window.device_pixel_ratio() as f32;
            conn.send_viewport_changed(width, height, scale).ok();
        }
    });
    if let Some(browser_window) = web_sys::window() {
        browser_window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .ok();
    }
    // Prevent the closure from being dropped (which would deregister the listener).
    closure.forget();
}

/// Attach keyboard event listeners on the document that forward key events
/// to the streaming server. These are global listeners — the server decides
/// which element receives focus.
fn setup_keyboard_listener(connection: &Rc<Connection>) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        // keydown
        {
            let conn = connection.clone();
            let closure =
                Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                    if let Ok(keyboard_event) = event.dyn_into::<web_sys::KeyboardEvent>() {
                        let modifiers =
                            connection::modifiers_from_keyboard_event(&keyboard_event);
                        conn.send_key_down(keyboard_event.key(), modifiers).ok();
                    }
                });
            document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .ok();
            closure.forget();
        }

        // keyup
        {
            let conn = connection.clone();
            let closure =
                Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                    if let Ok(keyboard_event) = event.dyn_into::<web_sys::KeyboardEvent>() {
                        let modifiers =
                            connection::modifiers_from_keyboard_event(&keyboard_event);
                        conn.send_key_up(keyboard_event.key(), modifiers).ok();
                    }
                });
            document
                .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
                .ok();
            closure.forget();
        }
    }
}

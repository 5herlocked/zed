use wasm_bindgen::prelude::*;

mod connection;
mod remote_view;

use connection::Connection;
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

    let app = Application::new();
    app.run(move |cx| {
        let (frame_tx, mut frame_rx) = futures::channel::mpsc::unbounded::<WireFrame>();

        let connection = Rc::new(
            Connection::connect(&url, frame_tx)
                .expect("failed to connect WebSocket"),
        );

        let conn = connection.clone();
        let window = cx
            .open_window(WindowOptions::default(), |_window, cx| {
                cx.new(|_cx| RemoteView::new(conn))
            })
            .expect("failed to open window");

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

        log::info!("zed_web: launched, connected to server");
    });

    Ok(())
}

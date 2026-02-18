//! Launch mode for the web streaming platform.
//!
//! This module provides the public entry point `init_web_streaming` that the
//! Zed application calls during startup. When the `ZED_WEB_STREAMING`
//! environment variable is set, it:
//!
//! 1. Starts a WebSocket server on a configurable port (default 3101).
//! 2. Registers a scene observer factory on `App` so that every window
//!    automatically captures and broadcasts its rendered scene on each frame.
//!
//! The scene observer collects ALL atlas tile pixel data referenced by sprites
//! in the current frame and includes them as a full snapshot in every frame
//! message. The frame is serialized using a compact binary format (see
//! `binary_frame.rs`) and sent as a binary WebSocket message. No JSON overhead.
//!
//! Usage from the Zed main function:
//!
//! ```ignore
//! app.run(move |cx| {
//!     gpui::init_web_streaming(cx);
//!     // ... rest of initialization
//! });
//! ```

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::{PlatformAtlas, Scene};

use super::mirroring_atlas::{CachedTileData, MirroringAtlas};
use super::proto_encoding::encode_frame;
use super::server::WebStreamingServer;

/// Default address for the web streaming WebSocket server.
const DEFAULT_LISTEN_ADDR: &str = "0.0.0.0:3101";

/// Environment variable to enable web streaming mode.
const ENV_WEB_STREAMING: &str = "ZED_WEB_STREAMING";

/// Environment variable to override the listen address.
const ENV_WEB_STREAMING_ADDR: &str = "ZED_WEB_STREAMING_ADDR";

/// Check whether web streaming mode is requested via environment variable.
fn is_web_streaming_requested() -> bool {
    std::env::var(ENV_WEB_STREAMING)
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Get the configured listen address, falling back to the default.
fn listen_addr() -> String {
    std::env::var(ENV_WEB_STREAMING_ADDR).unwrap_or_else(|_| DEFAULT_LISTEN_ADDR.to_string())
}

/// Initialize web streaming if the `ZED_WEB_STREAMING` environment variable
/// is set to `1` or `true`.
///
/// This starts a WebSocket server on the configured address and registers a
/// scene observer on every window that opens. Each frame presentation
/// serializes the scene to a compact binary format and broadcasts it to all
/// connected browser clients.
///
/// Call this early in `app.run()`, before any windows are opened.
///
/// Does nothing if `ZED_WEB_STREAMING` is not set.
pub fn init_web_streaming(cx: &mut crate::App) {
    if !is_web_streaming_requested() {
        return;
    }

    let addr = listen_addr();
    log::info!("Web streaming enabled, starting server on {}", addr);

    // Start the server synchronously using smol::block_on. This runs on the
    // current thread briefly to bind the TCP listener before any windows open.
    let server = smol::block_on(async { WebStreamingServer::start(&addr).await });

    let server = match server {
        Ok(s) => {
            log::info!("Web streaming server listening on ws://{}", s.listen_addr());
            Arc::new(s)
        }
        Err(err) => {
            log::error!("Failed to start web streaming server: {}", err);
            return;
        }
    };

    let frame_counter = Arc::new(AtomicU64::new(0));

    // Register a factory that creates a scene observer for each new window.
    // The observer serializes the scene as binary and broadcasts it over WebSocket.
    let factory_server = server.clone();
    let factory_counter = frame_counter.clone();

    cx.set_scene_observer_factory(Arc::new(move |atlas: Arc<dyn PlatformAtlas>| {
        let server = factory_server.clone();
        let counter = factory_counter.clone();

        // Downcast the atlas to MirroringAtlas so we can drain frame tiles.
        // When web streaming is enabled, Window::new wraps the platform atlas
        // in a MirroringAtlas, so this downcast should always succeed.
        let mirroring: Option<Arc<MirroringAtlas>> = arc_downcast(atlas);

        Box::new(
            move |scene: &Scene, viewport_size: crate::Size<crate::Pixels>, scale_factor: f32| {
                let frame_id = counter.fetch_add(1, Ordering::Relaxed);

                // Drain all tile data captured during this frame's paint phase.
                // The MirroringAtlas always runs the build callback, so this
                // contains every tile that was rasterized this frame.
                let atlas_tiles: Vec<CachedTileData> = if let Some(ref mirror) = mirroring {
                    mirror.take_frame_tiles()
                } else {
                    vec![]
                };

                let viewport_width = viewport_size.width.0 * scale_factor;
                let viewport_height = viewport_size.height.0 * scale_factor;

                // Encode the frame as protobuf.
                let bytes = encode_frame(
                    frame_id,
                    viewport_width,
                    viewport_height,
                    scale_factor,
                    &atlas_tiles,
                    scene,
                );

                // Broadcast to all connected browser clients as a binary message.
                server.broadcast_binary(&bytes);
            },
        )
    }));
}

/// Attempt to downcast an `Arc<dyn PlatformAtlas>` to `Arc<MirroringAtlas>`.
///
/// Returns `None` if the underlying type is not `MirroringAtlas`.
fn arc_downcast(atlas: Arc<dyn PlatformAtlas>) -> Option<Arc<MirroringAtlas>> {
    if atlas.as_any().is::<MirroringAtlas>() {
        let ptr = Arc::into_raw(atlas) as *const MirroringAtlas;
        // Safety: we just verified the concrete type via as_any().is::<MirroringAtlas>().
        Some(unsafe { Arc::from_raw(ptr) })
    } else {
        None
    }
}

//! A virtual window that implements `PlatformWindow` for the web streaming backend.
//!
//! This window runs the full GPUI element tree, layout, and paint cycle but
//! instead of rendering to a GPU surface, it captures the `Scene` at draw time
//! and makes it available for serialization and streaming to a browser client.
//!
//! The window maintains its own `StreamingAtlas` for sprite tile storage and
//! supports input injection from the browser via the standard GPUI input
//! dispatch path.
//!
//! Modeled after `TestWindow` in `platform/test/window.rs` but designed for
//! production use in headless web-streaming mode.

use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WindowHandle,
};

use crate::{
    Bounds, Corners, DispatchEventResult, GpuSpecs, Modifiers, Pixels, PlatformAtlas,
    PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow, Point, PromptButton,
    PromptLevel, RequestFrameOptions, Scene, Size, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowControls, WindowParams,
};

use super::streaming_atlas::StreamingAtlas;

// ---------------------------------------------------------------------------
// Scene callback
// ---------------------------------------------------------------------------

/// Signature for the callback that receives the scene after each draw call.
/// The streaming transport layer registers this to serialize and send frames.
pub type SceneCallback = Box<dyn FnMut(&Scene) + Send>;

// ---------------------------------------------------------------------------
// StreamingWindow
// ---------------------------------------------------------------------------

pub(crate) struct StreamingWindowState {
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    title: String,
    active: bool,
    fullscreen: bool,
    appearance: WindowAppearance,

    sprite_atlas: Arc<StreamingAtlas>,

    input_handler: Option<PlatformInputHandler>,
    input_callback: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    request_frame_callback: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    active_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    hover_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved_callback: Option<Box<dyn FnMut()>>,
    should_close_callback: Option<Box<dyn FnMut() -> bool>>,
    close_callback: Option<Box<dyn FnOnce()>>,
    appearance_changed_callback: Option<Box<dyn FnMut()>>,
    hit_test_callback: Option<Box<dyn FnMut() -> Option<WindowControlArea>>>,

    /// Called with the scene on every `draw()`. The streaming transport sets
    /// this to serialize the scene and send it over WebSocket.
    scene_callback: Option<SceneCallback>,
}

#[derive(Clone)]
pub struct StreamingWindow(pub(crate) Rc<Mutex<StreamingWindowState>>);

impl StreamingWindow {
    pub fn new(params: WindowParams, atlas: Arc<StreamingAtlas>) -> Self {
        let scale_factor = 2.0; // Default; the browser will send the real value.

        Self(Rc::new(Mutex::new(StreamingWindowState {
            bounds: params.bounds,
            scale_factor,
            title: String::new(),
            active: true,
            fullscreen: false,
            appearance: WindowAppearance::Dark,

            sprite_atlas: atlas,

            input_handler: None,
            input_callback: None,
            request_frame_callback: None,
            active_status_change_callback: None,
            hover_status_change_callback: None,
            resize_callback: None,
            moved_callback: None,
            should_close_callback: None,
            close_callback: None,
            appearance_changed_callback: None,
            hit_test_callback: None,

            scene_callback: None,
        })))
    }

    /// Register a callback that receives the scene on every draw call.
    /// The web streaming transport uses this to serialize and broadcast frames.
    pub fn on_scene(&self, callback: SceneCallback) {
        self.0.lock().scene_callback = Some(callback);
    }

    /// Inject a browser input event into the GPUI dispatch pipeline.
    /// Returns true if the event was consumed.
    pub fn dispatch_input(&self, event: PlatformInput) -> bool {
        let mut lock = self.0.lock();
        let Some(mut callback) = lock.input_callback.take() else {
            return false;
        };
        drop(lock);
        let result = callback(event);
        self.0.lock().input_callback = Some(callback);
        !result.propagate
    }

    /// Update the viewport size from a browser resize event.
    pub fn set_size(&self, size: Size<Pixels>, scale_factor: f32) {
        let mut lock = self.0.lock();
        lock.bounds.size = size;
        lock.scale_factor = scale_factor;

        let callback = lock.resize_callback.take();
        drop(lock);

        if let Some(mut cb) = callback {
            cb(size, scale_factor);
            self.0.lock().resize_callback = Some(cb);
        }
    }

    /// Update the active (focused) state from the browser.
    pub fn set_active(&self, active: bool) {
        let mut lock = self.0.lock();
        lock.active = active;

        let callback = lock.active_status_change_callback.take();
        drop(lock);

        if let Some(mut cb) = callback {
            cb(active);
            self.0.lock().active_status_change_callback = Some(cb);
        }
    }

    /// Request a new frame to be drawn. Call this when the browser is ready
    /// for another frame or when input was received that may cause a redraw.
    pub fn request_frame(&self) {
        let mut lock = self.0.lock();
        let callback = lock.request_frame_callback.take();
        drop(lock);

        if let Some(mut cb) = callback {
            cb(RequestFrameOptions {
                require_presentation: false,
                force_render: true,
            });
            self.0.lock().request_frame_callback = Some(cb);
        }
    }

    /// Access the streaming atlas for reading deltas.
    pub fn atlas(&self) -> Arc<StreamingAtlas> {
        self.0.lock().sprite_atlas.clone()
    }
}

// ---------------------------------------------------------------------------
// HasWindowHandle / HasDisplayHandle
//
// The streaming window is not backed by a real OS window. We provide stub
// handles that identify it as a web-based virtual window. These handles are
// never passed to a windowing system -- they exist only to satisfy the trait
// bounds on PlatformWindow.
// ---------------------------------------------------------------------------

impl HasWindowHandle for StreamingWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        // SAFETY: We create a null-like handle that is never dereferenced.
        // This is only used to satisfy trait bounds; the streaming backend
        // never passes it to platform windowing APIs.
        Err(HandleError::Unavailable)
    }
}

impl HasDisplayHandle for StreamingWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Err(HandleError::Unavailable)
    }
}

// ---------------------------------------------------------------------------
// PlatformWindow implementation
// ---------------------------------------------------------------------------

impl PlatformWindow for StreamingWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.0.lock().bounds
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.bounds().size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        self.0.lock().bounds.size = size;
    }

    fn scale_factor(&self) -> f32 {
        self.0.lock().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        self.0.lock().appearance
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    fn modifiers(&self) -> Modifiers {
        Modifiers::default()
    }

    fn capslock(&self) -> crate::Capslock {
        crate::Capslock::default()
    }

    fn set_input_handler(&mut self, handler: PlatformInputHandler) {
        self.0.lock().input_handler = Some(handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.lock().input_handler.take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        // TODO: Forward prompts to the browser UI
        None
    }

    fn activate(&self) {
        self.set_active(true);
    }

    fn is_active(&self) -> bool {
        self.0.lock().active
    }

    fn is_hovered(&self) -> bool {
        // The browser client doesn't report hover state at the window level
        true
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.0.lock().title = title.to_owned();
    }

    fn set_background_appearance(&self, _background: WindowBackgroundAppearance) {
        // No-op: the browser always composites onto its own background
    }

    fn minimize(&self) {
        // No-op in streaming mode
    }

    fn zoom(&self) {
        // No-op in streaming mode
    }

    fn toggle_fullscreen(&self) {
        let mut lock = self.0.lock();
        lock.fullscreen = !lock.fullscreen;
    }

    fn is_fullscreen(&self) -> bool {
        self.0.lock().fullscreen
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.0.lock().request_frame_callback = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.0.lock().input_callback = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.lock().active_status_change_callback = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.lock().hover_status_change_callback = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.lock().resize_callback = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().moved_callback = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.lock().should_close_callback = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.0.lock().close_callback = Some(callback);
    }

    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
        self.0.lock().hit_test_callback = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().appearance_changed_callback = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        let mut lock = self.0.lock();
        if let Some(mut callback) = lock.scene_callback.take() {
            drop(lock);
            callback(scene);
            self.0.lock().scene_callback = Some(callback);
        }
    }

    fn completed_frame(&self) {
        // No-op: presentation is handled by the WebSocket transport
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.0.lock().sprite_atlas.clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        // The browser renderer uses a grayscale fallback for subpixel sprites
        // since WebGPU doesn't broadly support dual_source_blending. We still
        // return true so that GPUI produces SubpixelSprite primitives (which
        // the browser renders as grayscale-blended text).
        true
    }

    fn get_title(&self) -> String {
        self.0.lock().title.clone()
    }

    fn set_edited(&mut self, _edited: bool) {}

    fn show_character_palette(&self) {}

    fn titlebar_double_click(&self) {}

    fn set_app_id(&mut self, _app_id: &str) {}

    fn request_decorations(&self, _decorations: crate::WindowDecorations) {}

    fn show_window_menu(&self, _position: Point<Pixels>) {}

    fn start_window_move(&self) {}

    fn start_window_resize(&self, _edge: crate::ResizeEdge) {}

    fn window_decorations(&self) -> crate::Decorations {
        crate::Decorations::Server
    }

    fn window_controls(&self) -> WindowControls {
        WindowControls::default()
    }

    fn set_client_inset(&self, _inset: Pixels) {}

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        // No local GPU -- rendering happens in the browser
        None
    }
}

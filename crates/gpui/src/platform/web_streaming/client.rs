//! Web streaming client that extends the headless Linux client with window support.
//!
//! The standard `HeadlessClient` refuses to open windows because it has no display
//! server. This client wraps the same `LinuxCommon` infrastructure (event loop,
//! text system, executors) but returns `StreamingWindow` instances from `open_window`
//! instead of failing.
//!
//! Each `StreamingWindow` runs the full GPUI element tree, layout, and paint cycle
//! on the CPU. The resulting `Scene` is captured at draw time and made available for
//! serialization and streaming to the browser. No GPU or display server is required.
//!
//! The text system (`CosmicTextSystem` when compiled with `wayland` or `x11` features)
//! handles font loading and glyph rasterization on the CPU. Each window gets its own
//! `StreamingAtlas` that stores rasterized tiles in `Vec<u8>` buffers.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use calloop::{EventLoop, LoopHandle};
use util::ResultExt;

use crate::platform::linux::LinuxCommon;
use crate::platform::linux::platform::LinuxClient;
use crate::{
    AnyWindowHandle, CursorStyle, DisplayId, LinuxKeyboardLayout, PlatformDisplay,
    PlatformKeyboardLayout, PlatformWindow, WindowParams,
};

use super::streaming_atlas::StreamingAtlas;
use super::streaming_window::StreamingWindow;

// ---------------------------------------------------------------------------
// Client state
// ---------------------------------------------------------------------------

pub struct WebStreamingClientState {
    pub(crate) loop_handle: LoopHandle<'static, WebStreamingClient>,
    pub(crate) event_loop: Option<EventLoop<'static, WebStreamingClient>>,
    pub(crate) common: LinuxCommon,

    /// Open windows keyed by their handle. The streaming transport uses these
    /// to route input events from the browser to the correct window and to
    /// read back scenes for serialization.
    pub(crate) windows: HashMap<AnyWindowHandle, StreamingWindow>,

    /// The currently focused window, if any.
    pub(crate) active_window: Option<AnyWindowHandle>,
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct WebStreamingClient(pub(crate) Rc<RefCell<WebStreamingClientState>>);

impl WebStreamingClient {
    pub fn new() -> Self {
        let event_loop = EventLoop::try_new().unwrap();

        let (common, main_receiver) = LinuxCommon::new(event_loop.get_signal());

        let handle = event_loop.handle();

        handle
            .insert_source(main_receiver, |event, _, _: &mut WebStreamingClient| {
                if let calloop::channel::Event::Msg(runnable) = event {
                    runnable.run();
                }
            })
            .ok();

        WebStreamingClient(Rc::new(RefCell::new(WebStreamingClientState {
            event_loop: Some(event_loop),
            loop_handle: handle,
            common,
            windows: HashMap::new(),
            active_window: None,
        })))
    }

    /// Get a handle to the calloop event loop for registering additional
    /// event sources (e.g., a WebSocket server).
    pub fn loop_handle(&self) -> LoopHandle<'static, WebStreamingClient> {
        self.0.borrow().loop_handle.clone()
    }

    /// Get a reference to a streaming window by its handle.
    pub fn window(&self, handle: AnyWindowHandle) -> Option<StreamingWindow> {
        self.0.borrow().windows.get(&handle).cloned()
    }

    /// Iterate over all open streaming windows.
    pub fn windows(&self) -> Vec<(AnyWindowHandle, StreamingWindow)> {
        self.0
            .borrow()
            .windows
            .iter()
            .map(|(h, w)| (*h, w.clone()))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// LinuxClient implementation
// ---------------------------------------------------------------------------

impl LinuxClient for WebStreamingClient {
    fn with_common<R>(&self, f: impl FnOnce(&mut LinuxCommon) -> R) -> R {
        f(&mut self.0.borrow_mut().common)
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(LinuxKeyboardLayout::new("us".into()))
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        // No physical displays -- the browser is the display.
        vec![]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    fn display(&self, _id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    #[cfg(feature = "screen-capture")]
    fn is_screen_capture_supported(&self) -> bool {
        false
    }

    #[cfg(feature = "screen-capture")]
    fn screen_capture_sources(
        &self,
    ) -> futures::channel::oneshot::Receiver<anyhow::Result<Vec<Rc<dyn crate::ScreenCaptureSource>>>>
    {
        let (mut tx, rx) = futures::channel::oneshot::channel();
        tx.send(Err(anyhow::anyhow!(
            "Web streaming mode does not support screen capture."
        )))
        .ok();
        rx
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        self.0.borrow().active_window
    }

    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        let state = self.0.borrow();
        Some(state.windows.keys().copied().collect())
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        let atlas = Arc::new(StreamingAtlas::new());
        let window = StreamingWindow::new(params, atlas);

        let mut state = self.0.borrow_mut();
        state.windows.insert(handle, window.clone());
        state.active_window = Some(handle);

        Ok(Box::new(window))
    }

    fn compositor_name(&self) -> &'static str {
        "web-streaming"
    }

    fn set_cursor_style(&self, _style: CursorStyle) {
        // Cursor style is handled by the browser client
    }

    fn open_uri(&self, _uri: &str) {
        // TODO: Forward to browser via WebSocket
    }

    fn reveal_path(&self, _path: std::path::PathBuf) {
        // Not applicable in web streaming mode
    }

    fn write_to_primary(&self, _item: crate::ClipboardItem) {
        // TODO: Forward to browser clipboard via WebSocket
    }

    fn write_to_clipboard(&self, _item: crate::ClipboardItem) {
        // TODO: Forward to browser clipboard via WebSocket
    }

    fn read_from_primary(&self) -> Option<crate::ClipboardItem> {
        // TODO: Read from browser clipboard via WebSocket
        None
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        // TODO: Read from browser clipboard via WebSocket
        None
    }

    fn run(&self) {
        let mut event_loop = self
            .0
            .borrow_mut()
            .event_loop
            .take()
            .expect("App is already running");

        event_loop.run(None, &mut self.clone(), |_| {}).log_err();
    }
}

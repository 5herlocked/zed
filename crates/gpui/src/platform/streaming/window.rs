use crate::{
    AtlasKey, AtlasTextureId, AtlasTile, Bounds, DevicePixels, DispatchEventResult, GpuSpecs,
    Pixels, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow,
    Point, PromptButton, PromptLevel, RequestFrameOptions, Size, TileId, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea,
};
use crate::display_tree::DisplayTree;
use collections::HashMap;
use parking_lot::Mutex;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::sync::{self, Arc};

pub struct StreamingWindowState {
    bounds: Bounds<Pixels>,
    title: String,
    scale_factor: f32,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    input_handler: Option<PlatformInputHandler>,
    request_frame_callback: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    input_callback: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    active_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    hover_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved_callback: Option<Box<dyn FnMut()>>,
    should_close_callback: Option<Box<dyn FnMut() -> bool>>,
    close_callback: Option<Box<dyn FnOnce()>>,
    hit_test_callback: Option<Box<dyn FnMut() -> Option<WindowControlArea>>>,
    frame_tx: smol::channel::Sender<DisplayTree>,
}

#[derive(Clone)]
pub struct StreamingWindow(pub(crate) Arc<Mutex<StreamingWindowState>>);

impl HasWindowHandle for StreamingWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::NotSupported)
    }
}

impl HasDisplayHandle for StreamingWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::NotSupported)
    }
}

impl StreamingWindow {
    pub fn new(
        width: f32,
        height: f32,
        scale_factor: f32,
        frame_tx: smol::channel::Sender<DisplayTree>,
    ) -> Self {
        Self(Arc::new(Mutex::new(StreamingWindowState {
            bounds: Bounds {
                origin: Point::default(),
                size: Size {
                    width: Pixels(width),
                    height: Pixels(height),
                },
            },
            title: String::new(),
            scale_factor,
            sprite_atlas: Arc::new(StreamingAtlas::new()),
            input_handler: None,
            request_frame_callback: None,
            input_callback: None,
            active_status_change_callback: None,
            hover_status_change_callback: None,
            resize_callback: None,
            moved_callback: None,
            should_close_callback: None,
            close_callback: None,
            hit_test_callback: None,
            frame_tx,
        })))
    }

    /// Inject a resize event from the transport layer (browser viewport changed).
    pub fn simulate_resize(&self, width: f32, height: f32) {
        let scale_factor;
        let mut lock = self.0.lock();
        lock.bounds.size = Size {
            width: Pixels(width),
            height: Pixels(height),
        };
        scale_factor = lock.scale_factor;
        let callback = lock.resize_callback.take();
        drop(lock);
        if let Some(mut cb) = callback {
            cb(
                Size {
                    width: Pixels(width),
                    height: Pixels(height),
                },
                scale_factor,
            );
            self.0.lock().resize_callback = Some(cb);
        }
    }

    /// Inject an input event from the transport layer (browser forwarded an action).
    pub fn simulate_input(&self, event: PlatformInput) -> bool {
        let mut lock = self.0.lock();
        let callback = lock.input_callback.take();
        drop(lock);
        if let Some(mut cb) = callback {
            let result = cb(event);
            self.0.lock().input_callback = Some(cb);
            !result.propagate
        } else {
            false
        }
    }

    /// Trigger a frame request. The transport layer calls this at its
    /// desired frame rate (e.g. 60Hz driven by connected browser rAF).
    pub fn request_frame(&self) {
        let mut lock = self.0.lock();
        let callback = lock.request_frame_callback.take();
        drop(lock);
        if let Some(mut cb) = callback {
            cb(RequestFrameOptions {
                require_presentation: true,
            });
            self.0.lock().request_frame_callback = Some(cb);
        }
    }

    /// Send a captured display tree to the transport layer.
    pub fn send_frame(&self, tree: DisplayTree) {
        let lock = self.0.lock();
        let _ = lock.frame_tx.try_send(tree);
    }
}

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
        WindowAppearance::Dark
    }

    fn display(&self) -> Option<std::rc::Rc<dyn PlatformDisplay>> {
        None
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    fn modifiers(&self) -> crate::Modifiers {
        crate::Modifiers::default()
    }

    fn capslock(&self) -> crate::Capslock {
        crate::Capslock::default()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.lock().input_handler = Some(input_handler);
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
        None
    }

    fn activate(&self) {}

    fn is_active(&self) -> bool {
        true
    }

    fn is_hovered(&self) -> bool {
        false
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.0.lock().title = title.to_owned();
    }

    fn set_background_appearance(&self, _background: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        false
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

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn draw(&self, _scene: &crate::Scene) {
        // Scene rendering is a no-op. The display tree capture in
        // Window::draw() handles serialization to the transport layer.
    }

    fn sprite_atlas(&self) -> sync::Arc<dyn PlatformAtlas> {
        self.0.lock().sprite_atlas.clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        false
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }
}

struct StreamingAtlasState {
    next_id: u32,
    tiles: HashMap<AtlasKey, AtlasTile>,
}

struct StreamingAtlas(Mutex<StreamingAtlasState>);

impl StreamingAtlas {
    fn new() -> Self {
        Self(Mutex::new(StreamingAtlasState {
            next_id: 0,
            tiles: HashMap::default(),
        }))
    }
}

impl PlatformAtlas for StreamingAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> anyhow::Result<
            Option<(Size<DevicePixels>, std::borrow::Cow<'a, [u8]>)>,
        >,
    ) -> anyhow::Result<Option<AtlasTile>> {
        let mut state = self.0.lock();
        if let Some(tile) = state.tiles.get(key) {
            return Ok(Some(tile.clone()));
        }
        drop(state);

        let Some((size, _)) = build()? else {
            return Ok(None);
        };

        let mut state = self.0.lock();
        state.next_id += 1;
        let texture_id = state.next_id;
        state.next_id += 1;
        let tile_id = state.next_id;

        state.tiles.insert(
            key.clone(),
            AtlasTile {
                texture_id: AtlasTextureId {
                    index: texture_id,
                    kind: crate::AtlasTextureKind::Monochrome,
                },
                tile_id: TileId(tile_id),
                padding: 0,
                bounds: Bounds {
                    origin: Point::default(),
                    size,
                },
            },
        );

        Ok(Some(state.tiles[key].clone()))
    }

    fn remove(&self, key: &AtlasKey) {
        self.0.lock().tiles.remove(key);
    }
}

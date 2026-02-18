use crate::{
    AnyWindowHandle, AtlasKey, AtlasTextureId, AtlasTile, Bounds, DevicePixels,
    DispatchEventResult, GpuSpecs, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, PromptButton, RequestFrameOptions, Scene, Size,
    TileId, WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControlArea,
    WindowParams,
};
use collections::HashMap;
use parking_lot::Mutex;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{
    borrow::Cow,
    rc::Rc,
    sync::Arc,
};

pub(crate) struct WebWindow {
    bounds: Bounds<Pixels>,
    display: Rc<dyn PlatformDisplay>,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    input_handler: Option<PlatformInputHandler>,
}

impl WebWindow {
    pub fn new(
        _handle: AnyWindowHandle,
        params: WindowParams,
        display: Rc<dyn PlatformDisplay>,
    ) -> Self {
        Self {
            bounds: params.bounds,
            display,
            sprite_atlas: Arc::new(WebAtlas::new()),
            input_handler: None,
        }
    }
}

impl HasWindowHandle for WebWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::NotSupported)
    }
}

impl HasDisplayHandle for WebWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::NotSupported)
    }
}

impl PlatformWindow for WebWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds)
    }

    fn content_size(&self) -> Size<Pixels> {
        self.bounds.size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        self.bounds.size = size;
    }

    fn scale_factor(&self) -> f32 {
        1.0
    }

    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
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
        self.input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.input_handler.take()
    }

    fn prompt(
        &self,
        _level: crate::PromptLevel,
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

    fn set_title(&mut self, _title: &str) {}

    fn set_background_appearance(&self, _background: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        false
    }

    fn on_request_frame(&self, _callback: Box<dyn FnMut(RequestFrameOptions)>) {}

    fn on_input(&self, _callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {}

    fn on_active_status_change(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_hover_status_change(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_resize(&self, _callback: Box<dyn FnMut(Size<Pixels>, f32)>) {}

    fn on_moved(&self, _callback: Box<dyn FnMut()>) {}

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {}

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {}

    fn on_hit_test_window_control(
        &self,
        _callback: Box<dyn FnMut() -> Option<WindowControlArea>>,
    ) {
    }

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn draw(&self, _scene: &Scene) {}

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.sprite_atlas.clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        false
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }
}

struct WebAtlasState {
    next_id: u32,
    tiles: HashMap<AtlasKey, AtlasTile>,
}

struct WebAtlas(Mutex<WebAtlasState>);

impl WebAtlas {
    fn new() -> Self {
        WebAtlas(Mutex::new(WebAtlasState {
            next_id: 0,
            tiles: HashMap::default(),
        }))
    }
}

impl PlatformAtlas for WebAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> anyhow::Result<Option<(Size<DevicePixels>, Cow<'a, [u8]>)>>,
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

        let tile = AtlasTile {
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
        };

        state.tiles.insert(key.clone(), tile.clone());
        Ok(Some(tile))
    }

    fn remove(&self, key: &AtlasKey) {
        self.0.lock().tiles.remove(key);
    }
}

use crate::{
    AnyWindowHandle, AtlasKey, AtlasTextureId, AtlasTile, Bounds, DevicePixels,
    DispatchEventResult, GpuSpecs, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, PromptButton, RequestFrameOptions, Scene, Size,
    TileId, WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControlArea,
    WindowParams, px,
};
use collections::HashMap;
use parking_lot::Mutex;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{
    borrow::Cow,
    cell::RefCell,
    rc::Rc,
    sync::Arc,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

struct WebWindowCallbacks {
    request_frame: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    input: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    hover_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

struct WebWindowState {
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    mouse_position: Point<Pixels>,
    is_hovered: bool,
    title: String,
    callbacks: WebWindowCallbacks,
    input_handler: Option<PlatformInputHandler>,
    canvas: Option<web_sys::HtmlCanvasElement>,
    raf_handle: Option<i32>,
}

pub(crate) struct WebWindow {
    state: Rc<RefCell<WebWindowState>>,
    display: Rc<dyn PlatformDisplay>,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    // Prevent closures from being dropped while event listeners are active.
    _event_closures: Rc<RefCell<Vec<Closure<dyn FnMut(web_sys::Event)>>>>,
}

impl WebWindow {
    pub fn new(
        _handle: AnyWindowHandle,
        params: WindowParams,
        display: Rc<dyn PlatformDisplay>,
    ) -> Self {
        let scale_factor = web_sys::window()
            .map(|w| w.device_pixel_ratio() as f32)
            .unwrap_or(1.0);

        let canvas = Self::create_canvas(&params, scale_factor);

        let state = Rc::new(RefCell::new(WebWindowState {
            bounds: params.bounds,
            scale_factor,
            mouse_position: Point::default(),
            is_hovered: false,
            title: String::new(),
            callbacks: WebWindowCallbacks {
                request_frame: None,
                input: None,
                active_status_change: None,
                hover_status_change: None,
                resize: None,
                moved: None,
                should_close: None,
                close: None,
                appearance_changed: None,
            },
            input_handler: None,
            canvas: canvas.clone(),
            raf_handle: None,
        }));

        let event_closures = Rc::new(RefCell::new(Vec::new()));

        if let Some(ref canvas) = canvas {
            Self::attach_event_listeners(canvas, &state, &event_closures);
        }

        Self {
            state,
            display,
            sprite_atlas: Arc::new(WebAtlas::new()),
            _event_closures: event_closures,
        }
    }

    fn create_canvas(
        params: &WindowParams,
        scale_factor: f32,
    ) -> Option<web_sys::HtmlCanvasElement> {
        let document = web_sys::window()?.document()?;
        let canvas = document
            .create_element("canvas")
            .ok()?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .ok()?;

        let width = params.bounds.size.width.0;
        let height = params.bounds.size.height.0;
        canvas.set_width((width * scale_factor) as u32);
        canvas.set_height((height * scale_factor) as u32);
        let style = canvas.style();
        style.set_property("width", &format!("{width}px")).ok()?;
        style.set_property("height", &format!("{height}px")).ok()?;

        document.body()?.append_child(&canvas).ok()?;
        Some(canvas)
    }

    fn attach_event_listeners(
        canvas: &web_sys::HtmlCanvasElement,
        state: &Rc<RefCell<WebWindowState>>,
        closures: &Rc<RefCell<Vec<Closure<dyn FnMut(web_sys::Event)>>>>,
    ) {
        // Mouse move
        {
            let state = state.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Ok(mouse_event) = event.dyn_into::<web_sys::MouseEvent>() {
                    let mut s = state.borrow_mut();
                    s.mouse_position = Point {
                        x: px(mouse_event.offset_x() as f32),
                        y: px(mouse_event.offset_y() as f32),
                    };
                    s.is_hovered = true;
                }
            });
            canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        // Mouse leave
        {
            let state = state.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
                let mut s = state.borrow_mut();
                s.is_hovered = false;
                if let Some(ref mut callback) = s.callbacks.hover_status_change {
                    callback(false);
                }
            });
            canvas
                .add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        // Mouse enter
        {
            let state = state.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
                let mut s = state.borrow_mut();
                s.is_hovered = true;
                if let Some(ref mut callback) = s.callbacks.hover_status_change {
                    callback(true);
                }
            });
            canvas
                .add_event_listener_with_callback("mouseenter", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }
    }

    fn start_animation_loop(state: &Rc<RefCell<WebWindowState>>) {
        let state = state.clone();
        fn schedule_frame(state: &Rc<RefCell<WebWindowState>>) {
            let state_clone = state.clone();
            let closure = Closure::once(move |_: f64| {
                {
                    let mut s = state_clone.borrow_mut();
                    if let Some(ref mut callback) = s.callbacks.request_frame {
                        callback(RequestFrameOptions {
                            require_presentation: false,
                            force_render: false,
                        });
                    }
                }
                schedule_frame(&state_clone);
            });
            if let Some(window) = web_sys::window() {
                let handle = window
                    .request_animation_frame(closure.as_ref().unchecked_ref())
                    .ok();
                state.borrow_mut().raf_handle = handle;
            }
            closure.forget();
        }
        schedule_frame(&state);
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
        self.state.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.state.borrow().bounds)
    }

    fn content_size(&self) -> Size<Pixels> {
        self.state.borrow().bounds.size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        let mut state = self.state.borrow_mut();
        state.bounds.size = size;
        if let Some(ref canvas) = state.canvas {
            let sf = state.scale_factor;
            canvas.set_width((size.width.0 * sf) as u32);
            canvas.set_height((size.height.0 * sf) as u32);
            let style = canvas.style();
            style
                .set_property("width", &format!("{}px", size.width.0))
                .ok();
            style
                .set_property("height", &format!("{}px", size.height.0))
                .ok();
        }
    }

    fn scale_factor(&self) -> f32 {
        self.state.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        // Detect browser dark mode preference.
        let prefers_dark = web_sys::window()
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
            .map(|mql| mql.matches())
            .unwrap_or(false);
        if prefers_dark {
            WindowAppearance::Dark
        } else {
            WindowAppearance::Light
        }
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.state.borrow().mouse_position
    }

    fn modifiers(&self) -> crate::Modifiers {
        crate::Modifiers::default()
    }

    fn capslock(&self) -> crate::Capslock {
        crate::Capslock::default()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.state.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.state.borrow_mut().input_handler.take()
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

    fn activate(&self) {
        if let Some(ref canvas) = self.state.borrow().canvas {
            canvas.focus().ok();
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_hovered(&self) -> bool {
        self.state.borrow().is_hovered
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.state.borrow_mut().title = title.to_string();
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            document.set_title(title);
        }
    }

    fn set_background_appearance(&self, _background: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {
        if let Some(ref canvas) = self.state.borrow().canvas {
            canvas.request_fullscreen().ok();
        }
    }

    fn is_fullscreen(&self) -> bool {
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.fullscreen_element())
            .is_some()
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.state.borrow_mut().callbacks.request_frame = Some(callback);
        Self::start_animation_loop(&self.state);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.state.borrow_mut().callbacks.input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.state.borrow_mut().callbacks.active_status_change = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.state.borrow_mut().callbacks.hover_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.state.borrow_mut().callbacks.resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().callbacks.moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.state.borrow_mut().callbacks.should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.state.borrow_mut().callbacks.close = Some(callback);
    }

    fn on_hit_test_window_control(
        &self,
        _callback: Box<dyn FnMut() -> Option<WindowControlArea>>,
    ) {
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().callbacks.appearance_changed = Some(callback);
    }

    fn draw(&self, _scene: &Scene) {
        // Scene rendering to Canvas2D will be implemented as part of the
        // browser-side hydration layer. For now, clear the canvas each frame.
        let state = self.state.borrow();
        if let Some(ref canvas) = state.canvas {
            if let Ok(Some(ctx)) = canvas.get_context("2d") {
                if let Ok(ctx) = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                    let width = canvas.width() as f64;
                    let height = canvas.height() as f64;
                    ctx.clear_rect(0.0, 0.0, width, height);
                }
            }
        }
    }

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

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

/// A text draw command captured during the paint phase for Canvas2D rendering.
#[derive(Clone)]
pub(crate) struct TextDraw {
    pub text: String,
    pub origin_x: f32,
    pub origin_y: f32,
    pub font_size: f32,
    pub color: crate::Hsla,
    pub font_family: String,
    pub font_weight: u16,
    pub font_style_italic: bool,
}

struct WebWindowState {
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    mouse_position: Point<Pixels>,
    is_hovered: bool,
    title: String,
    input_handler: Option<PlatformInputHandler>,
    canvas: Option<web_sys::HtmlCanvasElement>,
    text_overlay: Option<web_sys::HtmlDivElement>,
    raf_handle: Option<i32>,
    pending_text_draws: Vec<TextDraw>,
}

pub(crate) struct WebWindow {
    state: Rc<RefCell<WebWindowState>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    display: Rc<dyn PlatformDisplay>,
    sprite_atlas: Arc<WebAtlas>,
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
        let text_overlay = Self::create_text_overlay(&params, canvas.as_ref());

        let callbacks = Rc::new(RefCell::new(WebWindowCallbacks {
            request_frame: None,
            input: None,
            active_status_change: None,
            hover_status_change: None,
            resize: None,
            moved: None,
            should_close: None,
            close: None,
            appearance_changed: None,
        }));

        let state = Rc::new(RefCell::new(WebWindowState {
            bounds: params.bounds,
            scale_factor,
            mouse_position: Point::default(),
            is_hovered: false,
            title: String::new(),
            input_handler: None,
            canvas: canvas.clone(),
            text_overlay,
            raf_handle: None,
            pending_text_draws: Vec::new(),
        }));

        let event_closures = Rc::new(RefCell::new(Vec::new()));

        if let Some(ref canvas) = canvas {
            Self::attach_event_listeners(canvas, &state, &callbacks, &event_closures);
        }

        if let Some(ref overlay) = state.borrow().text_overlay {
            Self::attach_overlay_event_forwarding(overlay, &state, &callbacks, &event_closures);
        }

        // Listen for browser window resize events and forward them to GPUI.
        {
            let state = state.clone();
            let callbacks = callbacks.clone();
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
                    let scale_factor = browser_window.device_pixel_ratio() as f32;

                    let new_size = Size {
                        width: Pixels(width),
                        height: Pixels(height),
                    };

                    {
                        let mut s = state.borrow_mut();
                        s.bounds.size = new_size;
                        s.scale_factor = scale_factor;
                        if let Some(ref canvas) = s.canvas {
                            canvas.set_width((width * scale_factor) as u32);
                            canvas.set_height((height * scale_factor) as u32);
                        }
                    }

                    if let Some(ref mut cb) = callbacks.borrow_mut().resize {
                        cb(new_size, scale_factor);
                    }
                }
            });
            if let Some(browser_window) = web_sys::window() {
                browser_window
                    .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                    .ok();
            }
            event_closures.borrow_mut().push(closure);
        }

        Self {
            state,
            callbacks,
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
        style.set_property("width", "100%").ok()?;
        style.set_property("height", "100%").ok()?;
        style.set_property("display", "block").ok()?;

        // Wrap canvas in a positioned container so the text overlay can use
        // position:absolute relative to it.
        let wrapper = document
            .create_element("div")
            .ok()?
            .dyn_into::<web_sys::HtmlDivElement>()
            .ok()?;
        let wrapper_style = wrapper.style();
        wrapper_style.set_property("position", "relative").ok()?;
        wrapper_style.set_property("width", "100%").ok()?;
        wrapper_style.set_property("height", "100%").ok()?;
        wrapper.append_child(&canvas).ok()?;
        document.body()?.append_child(&wrapper).ok()?;

        Some(canvas)
    }

    /// Create a transparent DOM overlay that sits on top of the canvas.
    /// Text spans are placed here so the browser can natively select them.
    /// The container has `pointer-events: none` so non-text clicks pass
    /// through to the canvas. Individual text spans override this with
    /// `pointer-events: auto` to enable text selection.
    fn create_text_overlay(
        _params: &WindowParams,
        canvas: Option<&web_sys::HtmlCanvasElement>,
    ) -> Option<web_sys::HtmlDivElement> {
        let document = web_sys::window()?.document()?;
        let overlay = document
            .create_element("div")
            .ok()?
            .dyn_into::<web_sys::HtmlDivElement>()
            .ok()?;

        let style = overlay.style();
        style.set_property("position", "absolute").ok()?;
        style.set_property("top", "0").ok()?;
        style.set_property("left", "0").ok()?;
        style.set_property("width", "100%").ok()?;
        style.set_property("height", "100%").ok()?;
        style.set_property("overflow", "hidden").ok()?;
        // Container doesn't capture pointer events; individual text spans do.
        style.set_property("pointer-events", "none").ok()?;
        style.set_property("user-select", "text").ok()?;
        style.set_property("-webkit-user-select", "text").ok()?;

        // Append to the canvas wrapper (positioned container) so absolute
        // positioning is relative to the canvas.
        if let Some(canvas) = canvas {
            if let Some(parent) = canvas.parent_node() {
                parent.append_child(&overlay).ok()?;
            }
        } else {
            document.body()?.append_child(&overlay).ok()?;
        }

        Some(overlay)
    }

    fn attach_overlay_event_forwarding(
        overlay: &web_sys::HtmlDivElement,
        state: &Rc<RefCell<WebWindowState>>,
        callbacks: &Rc<RefCell<WebWindowCallbacks>>,
        closures: &Rc<RefCell<Vec<Closure<dyn FnMut(web_sys::Event)>>>>,
    ) {
        let overlay_el: &web_sys::EventTarget = overlay.as_ref();

        {
            let state = state.clone();
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Ok(mouse_event) = event.dyn_into::<web_sys::MouseEvent>() {
                    let position = overlay_relative_position(&mouse_event, &state);
                    let modifiers = web_modifiers(&mouse_event);
                    let button = web_mouse_button(mouse_event.button());
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::MouseDown(crate::MouseDownEvent {
                            button, position, modifiers, click_count: 1, first_mouse: false,
                        }));
                    }
                }
            });
            overlay_el
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        {
            let state = state.clone();
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Ok(mouse_event) = event.dyn_into::<web_sys::MouseEvent>() {
                    let position = overlay_relative_position(&mouse_event, &state);
                    let modifiers = web_modifiers(&mouse_event);
                    let button = web_mouse_button(mouse_event.button());
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::MouseUp(crate::MouseUpEvent {
                            button, position, modifiers, click_count: 1,
                        }));
                    }
                }
            });
            overlay_el
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        {
            let state = state.clone();
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Ok(mouse_event) = event.dyn_into::<web_sys::MouseEvent>() {
                    let position = overlay_relative_position(&mouse_event, &state);
                    let modifiers = web_modifiers(&mouse_event);
                    state.borrow_mut().mouse_position = position;
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::MouseMove(crate::MouseMoveEvent {
                            position, pressed_button: None, modifiers,
                        }));
                    }
                }
            });
            overlay_el
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        {
            let state = state.clone();
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                event.prevent_default();
                if let Ok(wheel_event) = event.dyn_into::<web_sys::WheelEvent>() {
                    let mouse_event: &web_sys::MouseEvent = wheel_event.as_ref();
                    let position = overlay_relative_position(mouse_event, &state);
                    let modifiers = web_modifiers(mouse_event);
                    let delta = crate::ScrollDelta::Pixels(Point {
                        x: px(-wheel_event.delta_x() as f32),
                        y: px(-wheel_event.delta_y() as f32),
                    });
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::ScrollWheel(crate::ScrollWheelEvent {
                            position, delta, modifiers,
                            touch_phase: crate::TouchPhase::Moved,
                        }));
                    }
                }
            });
            overlay_el
                .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }
    }

    fn attach_event_listeners(
        canvas: &web_sys::HtmlCanvasElement,
        state: &Rc<RefCell<WebWindowState>>,
        callbacks: &Rc<RefCell<WebWindowCallbacks>>,
        closures: &Rc<RefCell<Vec<Closure<dyn FnMut(web_sys::Event)>>>>,
    ) {
        canvas.set_tab_index(0);

        // Mouse move
        {
            let state = state.clone();
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Ok(mouse_event) = event.dyn_into::<web_sys::MouseEvent>() {
                    let position = Point {
                        x: px(mouse_event.offset_x() as f32),
                        y: px(mouse_event.offset_y() as f32),
                    };
                    let modifiers = web_modifiers(&mouse_event);
                    {
                        let mut s = state.borrow_mut();
                        s.mouse_position = position;
                        s.is_hovered = true;
                    }
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::MouseMove(crate::MouseMoveEvent {
                            position, pressed_button: None, modifiers,
                        }));
                    }
                }
            });
            canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        // Mouse down
        {
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Ok(mouse_event) = event.dyn_into::<web_sys::MouseEvent>() {
                    let position = Point {
                        x: px(mouse_event.offset_x() as f32),
                        y: px(mouse_event.offset_y() as f32),
                    };
                    let modifiers = web_modifiers(&mouse_event);
                    let button = web_mouse_button(mouse_event.button());
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::MouseDown(crate::MouseDownEvent {
                            button, position, modifiers, click_count: 1, first_mouse: false,
                        }));
                    }
                }
            });
            canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        // Mouse up
        {
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Ok(mouse_event) = event.dyn_into::<web_sys::MouseEvent>() {
                    let position = Point {
                        x: px(mouse_event.offset_x() as f32),
                        y: px(mouse_event.offset_y() as f32),
                    };
                    let modifiers = web_modifiers(&mouse_event);
                    let button = web_mouse_button(mouse_event.button());
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::MouseUp(crate::MouseUpEvent {
                            button, position, modifiers, click_count: 1,
                        }));
                    }
                }
            });
            canvas
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        // Scroll wheel
        {
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                event.prevent_default();
                if let Ok(wheel_event) = event.dyn_into::<web_sys::WheelEvent>() {
                    let mouse_event: &web_sys::MouseEvent = wheel_event.as_ref();
                    let position = Point {
                        x: px(mouse_event.offset_x() as f32),
                        y: px(mouse_event.offset_y() as f32),
                    };
                    let modifiers = web_modifiers(mouse_event);
                    let delta = crate::ScrollDelta::Pixels(Point {
                        x: px(-wheel_event.delta_x() as f32),
                        y: px(-wheel_event.delta_y() as f32),
                    });
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::ScrollWheel(crate::ScrollWheelEvent {
                            position, delta, modifiers,
                            touch_phase: crate::TouchPhase::Moved,
                        }));
                    }
                }
            });
            canvas
                .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        // Keyboard down
        {
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Ok(keyboard_event) = event.dyn_into::<web_sys::KeyboardEvent>() {
                    keyboard_event.prevent_default();
                    let keystroke = web_keystroke(&keyboard_event);
                    let is_held = keyboard_event.repeat();
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::KeyDown(crate::KeyDownEvent {
                            keystroke, is_held, prefer_character_input: false,
                        }));
                    }
                }
            });
            canvas
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        // Keyboard up
        {
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Ok(keyboard_event) = event.dyn_into::<web_sys::KeyboardEvent>() {
                    let keystroke = web_keystroke(&keyboard_event);
                    if let Some(ref mut cb) = callbacks.borrow_mut().input {
                        cb(PlatformInput::KeyUp(crate::KeyUpEvent { keystroke }));
                    }
                }
            });
            canvas
                .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }

        // Mouse leave
        {
            let state = state.clone();
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
                state.borrow_mut().is_hovered = false;
                if let Some(ref mut cb) = callbacks.borrow_mut().hover_status_change {
                    cb(false);
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
            let callbacks = callbacks.clone();
            let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
                state.borrow_mut().is_hovered = true;
                if let Some(ref mut cb) = callbacks.borrow_mut().hover_status_change {
                    cb(true);
                }
            });
            canvas
                .add_event_listener_with_callback("mouseenter", closure.as_ref().unchecked_ref())
                .ok();
            closures.borrow_mut().push(closure);
        }
    }

    fn start_animation_loop(
        state: &Rc<RefCell<WebWindowState>>,
        callbacks: &Rc<RefCell<WebWindowCallbacks>>,
    ) {
        let state = state.clone();
        let callbacks = callbacks.clone();
        fn schedule_frame(
            state: &Rc<RefCell<WebWindowState>>,
            callbacks: &Rc<RefCell<WebWindowCallbacks>>,
        ) {
            let state_clone = state.clone();
            let callbacks_clone = callbacks.clone();
            let closure = Closure::once(move |_: f64| {
                if let Some(ref mut cb) = callbacks_clone.borrow_mut().request_frame {
                    cb(RequestFrameOptions {
                        require_presentation: false,
                        force_render: false,
                    });
                }
                schedule_frame(&state_clone, &callbacks_clone);
            });
            if let Some(window) = web_sys::window() {
                let handle = window
                    .request_animation_frame(closure.as_ref().unchecked_ref())
                    .ok();
                state.borrow_mut().raf_handle = handle;
            }
            closure.forget();
        }
        schedule_frame(&state, &callbacks);
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
            // CSS sizing uses 100% to fill the viewport; only the pixel
            // buffer dimensions (set_width/set_height above) change.
        }
        if let Some(ref overlay) = state.text_overlay {
            let style = overlay.style();
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
        self.callbacks.borrow_mut().request_frame = Some(callback);
        Self::start_animation_loop(&self.state, &self.callbacks);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.borrow_mut().hover_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_hit_test_window_control(
        &self,
        _callback: Box<dyn FnMut() -> Option<WindowControlArea>>,
    ) {
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        let atlas = self.sprite_atlas.clone();
        let mut state = self.state.borrow_mut();
        let Some(ref canvas) = state.canvas else {
            return;
        };
        let Ok(Some(ctx)) = canvas.get_context("2d") else {
            return;
        };
        let Ok(ctx) = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() else {
            return;
        };

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        ctx.clear_rect(0.0, 0.0, width, height);

        // Walk paint_operations in order to respect layering.
        // All coordinates in the scene are in ScaledPixels (physical/device pixels),
        // matching the canvas's physical pixel dimensions.
        for operation in &scene.paint_operations {
            match operation {
                crate::scene::PaintOperation::StartLayer(clip) => {
                    ctx.save();
                    let x: f64 = clip.origin.x.into();
                    let y: f64 = clip.origin.y.into();
                    let w: f64 = clip.size.width.into();
                    let h: f64 = clip.size.height.into();
                    ctx.begin_path();
                    ctx.rect(x, y, w, h);
                    ctx.clip();
                }
                crate::scene::PaintOperation::EndLayer => {
                    ctx.restore();
                }
                crate::scene::PaintOperation::Primitive(primitive) => {
                    paint_primitive(&ctx, primitive, &atlas);
                }
            }
        }

        // Render text on canvas for visual display.
        // Text draws use logical pixels, so we scale by the device pixel ratio
        // to match the canvas's physical pixel dimensions.
        let scale = state.scale_factor as f64;
        for text_draw in &state.pending_text_draws {
            let color = hsla_to_css_string(text_draw.color);
            let font_size = text_draw.font_size * scale as f32;
            let weight = text_draw.font_weight;
            let style = if text_draw.font_style_italic {
                "italic"
            } else {
                "normal"
            };
            let family = if text_draw.font_family.is_empty() {
                "sans-serif"
            } else {
                &text_draw.font_family
            };
            let font = format!("{style} {weight} {font_size}px {family}");
            ctx.set_font(&font);
            ctx.set_fill_style_str(&color);
            ctx.set_text_baseline("top");
            ctx.fill_text(
                &text_draw.text,
                text_draw.origin_x as f64 * scale,
                text_draw.origin_y as f64 * scale,
            )
            .ok();
        }

        // Also render text as invisible DOM spans in the overlay so the
        // browser's native text selection works. The spans are positioned
        // identically to the canvas text but use transparent color — the
        // canvas provides the visual rendering while the DOM provides
        // selectability.
        let overlay = state.text_overlay.clone();
        let text_draws: Vec<TextDraw> = state.pending_text_draws.drain(..).collect();

        if let Some(overlay) = overlay {
            overlay.set_inner_html("");

            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                for text_draw in text_draws {
                    if let Ok(span) = document.create_element("span") {
                        let family = if text_draw.font_family.is_empty() {
                            "sans-serif"
                        } else {
                            &text_draw.font_family
                        };
                        let font_style = if text_draw.font_style_italic {
                            "italic"
                        } else {
                            "normal"
                        };
                        let span_style = format!(
                            "position:absolute;\
                             left:{}px;\
                             top:{}px;\
                             font-size:{}px;\
                             font-weight:{};\
                             font-style:{};\
                             font-family:{};\
                             color:transparent;\
                             white-space:pre;\
                             line-height:1;\
                             pointer-events:auto;\
                             user-select:text;\
                             -webkit-user-select:text;\
                             cursor:text;",
                            text_draw.origin_x,
                            text_draw.origin_y,
                            text_draw.font_size,
                            text_draw.font_weight,
                            font_style,
                            family,
                        );
                        span.set_attribute("style", &span_style).ok();
                        span.set_text_content(Some(&text_draw.text));
                        overlay.append_child(&span).ok();
                    }
                }
            }
        }
    }

    fn push_text_draw(&self, draw: TextDraw) {
        self.state.borrow_mut().pending_text_draws.push(draw);
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

/// Compute mouse position relative to the canvas from a mouse event that
/// may have been captured by the text overlay. Uses the canvas bounding
/// rect from the state to convert page coordinates.
fn overlay_relative_position(
    event: &web_sys::MouseEvent,
    state: &Rc<RefCell<WebWindowState>>,
) -> Point<Pixels> {
    let s = state.borrow();
    if let Some(ref canvas) = s.canvas {
        let rect = canvas.get_bounding_client_rect();
        Point {
            x: px((event.client_x() as f64 - rect.left()) as f32),
            y: px((event.client_y() as f64 - rect.top()) as f32),
        }
    } else {
        Point {
            x: px(event.client_x() as f32),
            y: px(event.client_y() as f32),
        }
    }
}

fn paint_primitive(
    ctx: &web_sys::CanvasRenderingContext2d,
    primitive: &crate::scene::Primitive,
    atlas: &WebAtlas,
) {
    match primitive {
        crate::scene::Primitive::Quad(quad) => {
            let x: f64 = quad.bounds.origin.x.into();
            let y: f64 = quad.bounds.origin.y.into();
            let w: f64 = quad.bounds.size.width.into();
            let h: f64 = quad.bounds.size.height.into();

            let bg_rgba: crate::Rgba = quad.background.solid.into();
            if bg_rgba.a > 0.001 {
                let color = hsla_to_css_string(quad.background.solid);
                ctx.set_fill_style_str(&color);

                let tl: f64 = quad.corner_radii.top_left.into();
                if tl > 0.0 {
                    rounded_rect(ctx, x, y, w, h, tl);
                    ctx.fill();
                } else {
                    ctx.fill_rect(x, y, w, h);
                }
            }

            let border_top: f64 = quad.border_widths.top.into();
            let border_rgba: crate::Rgba = quad.border_color.into();
            if border_top > 0.0 && border_rgba.a > 0.001 {
                let border_css = hsla_to_css_string(quad.border_color);
                ctx.set_stroke_style_str(&border_css);
                ctx.set_line_width(border_top);
                let tl: f64 = quad.corner_radii.top_left.into();
                if tl > 0.0 {
                    rounded_rect(ctx, x, y, w, h, tl);
                    ctx.stroke();
                } else {
                    ctx.stroke_rect(x, y, w, h);
                }
            }
        }
        crate::scene::Primitive::Shadow(shadow) => {
            let x: f64 = shadow.bounds.origin.x.into();
            let y: f64 = shadow.bounds.origin.y.into();
            let w: f64 = shadow.bounds.size.width.into();
            let h: f64 = shadow.bounds.size.height.into();
            let blur: f64 = shadow.blur_radius.into();
            let color = hsla_to_css_string(shadow.color);

            ctx.save();
            ctx.set_shadow_blur(blur);
            ctx.set_shadow_color(&color);
            ctx.set_fill_style_str("rgba(0,0,0,0)");
            ctx.fill_rect(x, y, w, h);
            ctx.restore();
        }
        crate::scene::Primitive::Underline(underline) => {
            let x: f64 = underline.bounds.origin.x.into();
            let y: f64 = underline.bounds.origin.y.into();
            let w: f64 = underline.bounds.size.width.into();
            let thickness: f64 = underline.thickness.into();
            let color = hsla_to_css_string(underline.color);

            ctx.set_stroke_style_str(&color);
            ctx.set_line_width(thickness);
            ctx.begin_path();
            ctx.move_to(x, y);
            ctx.line_to(x + w, y);
            ctx.stroke();
        }
        crate::scene::Primitive::MonochromeSprite(sprite) => {
            paint_monochrome_sprite(ctx, sprite, atlas);
        }
        crate::scene::Primitive::SubpixelSprite(sprite) => {
            paint_subpixel_sprite(ctx, sprite, atlas);
        }
        crate::scene::Primitive::PolychromeSprite(sprite) => {
            paint_polychrome_sprite(ctx, sprite, atlas);
        }
        crate::scene::Primitive::Path(path) => {
            paint_path(ctx, path);
        }
        crate::scene::Primitive::Surface(_) => {}
    }
}

fn colorize_alpha_mask(bytes: &[u8], pixel_count: usize, rgba: crate::Rgba) -> Option<Vec<u8>> {
    let r = (rgba.r * 255.0) as u8;
    let g = (rgba.g * 255.0) as u8;
    let b = (rgba.b * 255.0) as u8;

    let mut out = vec![0u8; pixel_count * 4];
    for i in 0..pixel_count {
        let alpha = if bytes.len() == pixel_count {
            *bytes.get(i)?
        } else if bytes.len() >= pixel_count * 4 {
            *bytes.get(i * 4 + 3)?
        } else {
            return None;
        };
        let blended = ((alpha as f32 / 255.0) * rgba.a * 255.0) as u8;
        out[i * 4] = r;
        out[i * 4 + 1] = g;
        out[i * 4 + 2] = b;
        out[i * 4 + 3] = blended;
    }
    Some(out)
}

fn draw_rgba_to_canvas(
    ctx: &web_sys::CanvasRenderingContext2d,
    rgba_data: &[u8],
    src_w: u32,
    src_h: u32,
    dest_x: f64,
    dest_y: f64,
    dest_w: f64,
    dest_h: f64,
) {
    if src_w == 0 || src_h == 0 {
        return;
    }
    let expected = (src_w * src_h * 4) as usize;
    if rgba_data.len() < expected {
        return;
    }

    let clamped = wasm_bindgen::Clamped(&rgba_data[..expected]);
    let Ok(image_data) =
        web_sys::ImageData::new_with_u8_clamped_array_and_sh(clamped, src_w, src_h)
    else {
        return;
    };

    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let Ok(el) = document.create_element("canvas") else {
        return;
    };
    let Ok(offscreen) = el.dyn_into::<web_sys::HtmlCanvasElement>() else {
        return;
    };
    offscreen.set_width(src_w);
    offscreen.set_height(src_h);
    let Ok(Some(obj)) = offscreen.get_context("2d") else {
        return;
    };
    let Ok(off_ctx) = obj.dyn_into::<web_sys::CanvasRenderingContext2d>() else {
        return;
    };
    off_ctx.put_image_data(&image_data, 0.0, 0.0).ok();
    ctx.draw_image_with_html_canvas_element_and_dw_and_dh(
        &offscreen, dest_x, dest_y, dest_w, dest_h,
    )
    .ok();
}

fn paint_monochrome_sprite(
    ctx: &web_sys::CanvasRenderingContext2d,
    sprite: &crate::scene::MonochromeSprite,
    atlas: &WebAtlas,
) {
    let Some(bytes) = atlas.get_pixel_data(sprite.tile.tile_id) else {
        return;
    };
    if bytes.is_empty() {
        return;
    }

    let tile_w = sprite.tile.bounds.size.width.0 as u32;
    let tile_h = sprite.tile.bounds.size.height.0 as u32;
    if tile_w == 0 || tile_h == 0 {
        return;
    }

    let rgba: crate::Rgba = sprite.color.into();
    let pixel_count = (tile_w * tile_h) as usize;
    let Some(rgba_data) = colorize_alpha_mask(&bytes, pixel_count, rgba) else {
        return;
    };

    let x: f64 = sprite.bounds.origin.x.into();
    let y: f64 = sprite.bounds.origin.y.into();
    let w: f64 = sprite.bounds.size.width.into();
    let h: f64 = sprite.bounds.size.height.into();
    draw_rgba_to_canvas(ctx, &rgba_data, tile_w, tile_h, x, y, w, h);
}

fn paint_subpixel_sprite(
    ctx: &web_sys::CanvasRenderingContext2d,
    sprite: &crate::scene::SubpixelSprite,
    atlas: &WebAtlas,
) {
    let Some(bytes) = atlas.get_pixel_data(sprite.tile.tile_id) else {
        return;
    };
    if bytes.is_empty() {
        return;
    }

    let tile_w = sprite.tile.bounds.size.width.0 as u32;
    let tile_h = sprite.tile.bounds.size.height.0 as u32;
    if tile_w == 0 || tile_h == 0 {
        return;
    }

    let rgba: crate::Rgba = sprite.color.into();
    let pixel_count = (tile_w * tile_h) as usize;
    let Some(rgba_data) = colorize_alpha_mask(&bytes, pixel_count, rgba) else {
        return;
    };

    let x: f64 = sprite.bounds.origin.x.into();
    let y: f64 = sprite.bounds.origin.y.into();
    let w: f64 = sprite.bounds.size.width.into();
    let h: f64 = sprite.bounds.size.height.into();
    draw_rgba_to_canvas(ctx, &rgba_data, tile_w, tile_h, x, y, w, h);
}

fn paint_polychrome_sprite(
    ctx: &web_sys::CanvasRenderingContext2d,
    sprite: &crate::scene::PolychromeSprite,
    atlas: &WebAtlas,
) {
    let Some(bytes) = atlas.get_pixel_data(sprite.tile.tile_id) else {
        return;
    };
    if bytes.is_empty() {
        return;
    }

    let tile_w = sprite.tile.bounds.size.width.0 as u32;
    let tile_h = sprite.tile.bounds.size.height.0 as u32;
    if tile_w == 0 || tile_h == 0 {
        return;
    }

    let pixel_count = (tile_w * tile_h) as usize;
    let expected = pixel_count * 4;
    if bytes.len() < expected {
        return;
    }

    let mut rgba_data = bytes[..expected].to_vec();
    for i in 0..pixel_count {
        let off = i * 4;
        if sprite.grayscale {
            let r = rgba_data[off] as f32;
            let g = rgba_data[off + 1] as f32;
            let b = rgba_data[off + 2] as f32;
            let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            rgba_data[off] = gray;
            rgba_data[off + 1] = gray;
            rgba_data[off + 2] = gray;
        }
        rgba_data[off + 3] = (rgba_data[off + 3] as f32 * sprite.opacity) as u8;
    }

    let x: f64 = sprite.bounds.origin.x.into();
    let y: f64 = sprite.bounds.origin.y.into();
    let w: f64 = sprite.bounds.size.width.into();
    let h: f64 = sprite.bounds.size.height.into();
    draw_rgba_to_canvas(ctx, &rgba_data, tile_w, tile_h, x, y, w, h);
}

fn paint_path(
    ctx: &web_sys::CanvasRenderingContext2d,
    path: &crate::scene::Path<crate::ScaledPixels>,
) {
    if path.vertices.is_empty() {
        return;
    }

    let color = hsla_to_css_string(path.color.solid);
    ctx.set_fill_style_str(&color);

    ctx.begin_path();
    let mut i = 0;
    while i + 2 < path.vertices.len() {
        let v0 = &path.vertices[i];
        let v1 = &path.vertices[i + 1];
        let v2 = &path.vertices[i + 2];

        let x0: f64 = v0.xy_position.x.into();
        let y0: f64 = v0.xy_position.y.into();
        let x1: f64 = v1.xy_position.x.into();
        let y1: f64 = v1.xy_position.y.into();
        let x2: f64 = v2.xy_position.x.into();
        let y2: f64 = v2.xy_position.y.into();

        ctx.move_to(x0, y0);
        ctx.line_to(x1, y1);
        ctx.line_to(x2, y2);
        ctx.close_path();

        i += 3;
    }
    ctx.fill();
}

fn hsla_to_css_string(hsla: crate::Hsla) -> String {
    let rgba: crate::Rgba = hsla.into();
    format!(
        "rgba({},{},{},{})",
        (rgba.r * 255.0) as u8,
        (rgba.g * 255.0) as u8,
        (rgba.b * 255.0) as u8,
        rgba.a,
    )
}

fn rounded_rect(
    ctx: &web_sys::CanvasRenderingContext2d,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    radius: f64,
) {
    let r = radius.min(w / 2.0).min(h / 2.0);
    ctx.begin_path();
    ctx.move_to(x + r, y);
    ctx.line_to(x + w - r, y);
    ctx.arc_to(x + w, y, x + w, y + r, r).ok();
    ctx.line_to(x + w, y + h - r);
    ctx.arc_to(x + w, y + h, x + w - r, y + h, r).ok();
    ctx.line_to(x + r, y + h);
    ctx.arc_to(x, y + h, x, y + h - r, r).ok();
    ctx.line_to(x, y + r);
    ctx.arc_to(x, y, x + r, y, r).ok();
    ctx.close_path();
}

/// Convert browser mouse event modifiers to GPUI Modifiers.
fn web_modifiers(event: &web_sys::MouseEvent) -> crate::Modifiers {
    crate::Modifiers {
        control: event.ctrl_key(),
        alt: event.alt_key(),
        shift: event.shift_key(),
        platform: event.meta_key(),
        function: false,
    }
}

/// Convert browser mouse button index to GPUI MouseButton.
fn web_mouse_button(button: i16) -> crate::MouseButton {
    match button {
        0 => crate::MouseButton::Left,
        1 => crate::MouseButton::Middle,
        2 => crate::MouseButton::Right,
        3 => crate::MouseButton::Navigate(crate::NavigationDirection::Back),
        4 => crate::MouseButton::Navigate(crate::NavigationDirection::Forward),
        _ => crate::MouseButton::Left,
    }
}

/// Convert a browser KeyboardEvent to a GPUI Keystroke.
fn web_keystroke(event: &web_sys::KeyboardEvent) -> crate::Keystroke {
    let key = match event.key().as_str() {
        "ArrowUp" => "up".to_string(),
        "ArrowDown" => "down".to_string(),
        "ArrowLeft" => "left".to_string(),
        "ArrowRight" => "right".to_string(),
        "Backspace" => "backspace".to_string(),
        "Delete" => "delete".to_string(),
        "Enter" => "enter".to_string(),
        "Escape" => "escape".to_string(),
        "Tab" => "tab".to_string(),
        " " => "space".to_string(),
        "Home" => "home".to_string(),
        "End" => "end".to_string(),
        "PageUp" => "pageup".to_string(),
        "PageDown" => "pagedown".to_string(),
        "F1" => "f1".to_string(),
        "F2" => "f2".to_string(),
        "F3" => "f3".to_string(),
        "F4" => "f4".to_string(),
        "F5" => "f5".to_string(),
        "F6" => "f6".to_string(),
        "F7" => "f7".to_string(),
        "F8" => "f8".to_string(),
        "F9" => "f9".to_string(),
        "F10" => "f10".to_string(),
        "F11" => "f11".to_string(),
        "F12" => "f12".to_string(),
        other => other.to_lowercase(),
    };

    let key_char = if key.len() == 1 {
        Some(event.key())
    } else {
        None
    };

    crate::Keystroke {
        modifiers: crate::Modifiers {
            control: event.ctrl_key(),
            alt: event.alt_key(),
            shift: event.shift_key(),
            platform: event.meta_key(),
            function: false,
        },
        key,
        key_char,
    }
}

struct WebAtlasState {
    next_id: u32,
    tiles: HashMap<AtlasKey, AtlasTile>,
    pixel_data: HashMap<TileId, Vec<u8>>,
}

struct WebAtlas(Mutex<WebAtlasState>);

impl WebAtlas {
    fn new() -> Self {
        WebAtlas(Mutex::new(WebAtlasState {
            next_id: 0,
            tiles: HashMap::default(),
            pixel_data: HashMap::default(),
        }))
    }

    fn get_pixel_data(&self, tile_id: TileId) -> Option<Vec<u8>> {
        self.0.lock().pixel_data.get(&tile_id).cloned()
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

        let Some((size, bytes)) = build()? else {
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

        if !bytes.is_empty() {
            state.pixel_data.insert(TileId(tile_id), bytes.into_owned());
        }

        state.tiles.insert(key.clone(), tile.clone());
        Ok(Some(tile))
    }

    fn remove(&self, key: &AtlasKey) {
        let mut state = self.0.lock();
        if let Some(tile) = state.tiles.remove(key) {
            state.pixel_data.remove(&tile.tile_id);
        }
    }
}

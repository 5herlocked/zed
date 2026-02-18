use crate::display_tree::DisplayTree;
use crate::platform::streaming::StreamingWindow;
use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, ForegroundExecutor,
    Keymap, Menu, MenuItem, PathPromptOptions, Platform, PlatformDisplay, PlatformTextSystem,
    PlatformWindow, Task, ThermalState, WindowAppearance, WindowParams,
};
use anyhow::Result;
use futures::channel::oneshot;
use parking_lot::Mutex;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

/// Configuration for a streaming platform instance.
pub struct StreamingConfig {
    /// Viewport width in logical pixels.
    pub width: f32,
    /// Viewport height in logical pixels.
    pub height: f32,
    /// Display scale factor (1.0 for 1x, 2.0 for Retina).
    pub scale_factor: f32,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            width: 1280.0,
            height: 720.0,
            scale_factor: 2.0,
        }
    }
}

pub(crate) struct StreamingPlatformState {
    active_window: Option<AnyWindowHandle>,
    frame_tx: smol::channel::Sender<DisplayTree>,
    config: StreamingConfig,
    windows: Vec<StreamingWindow>,
}

/// A Platform implementation for headless-web streaming mode.
/// Creates StreamingWindows and provides real text layout via the native text system.
pub struct StreamingPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    state: Mutex<StreamingPlatformState>,
}

impl StreamingPlatform {
    pub(crate) fn new(
        config: StreamingConfig,
        frame_tx: smol::channel::Sender<DisplayTree>,
    ) -> Rc<Self> {
        #[cfg(target_os = "macos")]
        let dispatcher: Arc<dyn crate::PlatformDispatcher> =
            Arc::new(crate::platform::MacDispatcher::new());

        #[cfg(target_os = "macos")]
        let text_system: Arc<dyn PlatformTextSystem> =
            Arc::new(crate::platform::MacTextSystem::new());

        #[cfg(not(target_os = "macos"))]
        let text_system: Arc<dyn PlatformTextSystem> = Arc::new(crate::NoopTextSystem);

        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);

        Rc::new(Self {
            background_executor,
            foreground_executor,
            text_system,
            state: Mutex::new(StreamingPlatformState {
                active_window: None,
                frame_tx,
                config,
                windows: Vec::new(),
            }),
        })
    }
}

impl Platform for StreamingPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        on_finish_launching();

        // Install a frame-driving timer on the main run loop, analogous to
        // the CVDisplayLink that drives frames for native Mac windows.
        // Fires at ~30fps; each tick invokes request_frame() on every
        // registered StreamingWindow, which triggers GPUI's draw pipeline.
        #[cfg(target_os = "macos")]
        {
            use core_foundation::base::*;
            use core_foundation::date::*;
            use core_foundation::runloop::*;
            use std::ffi::c_void;

            let state_ptr: *const Mutex<StreamingPlatformState> = &self.state;

            extern "C" fn frame_timer_cb(_timer: CFRunLoopTimerRef, info: *mut c_void) {
                let state = unsafe { &*(info as *const Mutex<StreamingPlatformState>) };
                let windows: Vec<StreamingWindow> = state.lock().windows.clone();
                for window in &windows {
                    window.request_frame();
                }
            }

            unsafe {
                let interval = 1.0 / 120.0;
                let mut ctx: CFRunLoopTimerContext = std::mem::zeroed();
                ctx.info = state_ptr as *mut c_void;
                let timer = CFRunLoopTimerCreate(
                    kCFAllocatorDefault,
                    CFAbsoluteTimeGetCurrent() + interval,
                    interval,
                    0,
                    0,
                    frame_timer_cb,
                    &mut ctx,
                );
                CFRunLoopAddTimer(CFRunLoopGetMain(), timer, kCFRunLoopCommonModes);
            }
        }

        // Block the main thread and pump the foreground executor, matching
        // MacPlatform's headless behavior.  Without this, MacDispatcher's
        // GCD dispatches never execute and the process exits immediately.
        #[cfg(target_os = "macos")]
        unsafe {
            core_foundation::runloop::CFRunLoopRun();
        }
        #[cfg(not(target_os = "macos"))]
        {
            // On non-macOS, park the thread indefinitely. The foreground
            // executor uses a polling dispatcher that doesn't require a
            // run loop, so parking is sufficient to keep the process alive.
            std::thread::park();
        }
    }

    fn quit(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            core_foundation::runloop::CFRunLoopStop(core_foundation::runloop::CFRunLoopGetMain());
        }
        #[cfg(not(target_os = "macos"))]
        {
            // Unpark would need the main thread handle, which we don't store.
            // For now, quit on non-macOS is a no-op (process exits via signal).
        }
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![Rc::new(StreamingDisplay::new(&self.state.lock().config))]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(StreamingDisplay::new(&self.state.lock().config)))
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        self.state.lock().active_window
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        _params: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        let state = self.state.lock();
        let window = StreamingWindow::new(
            state.config.width,
            state.config.height,
            state.config.scale_factor,
            state.frame_tx.clone(),
        );
        let boxed = Box::new(window.clone());
        drop(state);
        let mut state = self.state.lock();
        state.active_window = Some(handle);
        state.windows.push(window);
        Ok(boxed)
    }

    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    fn open_url(&self, _url: &str) {}

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {}

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();
        let _ = tx.send(Ok(None));
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        let _ = tx.send(Ok(None));
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {}

    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {}

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn Action) -> bool>) {}

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }

    fn on_thermal_state_change(&self, _callback: Box<dyn FnMut()>) {}

    fn app_path(&self) -> Result<PathBuf> {
        Ok(std::env::current_exe()?)
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow::anyhow!(
            "no auxiliary executables in streaming mode"
        ))
    }

    fn set_cursor_style(&self, _style: CursorStyle) {}

    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        None
    }

    fn write_to_clipboard(&self, _item: ClipboardItem) {}

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn read_from_primary(&self) -> Option<ClipboardItem> {
        None
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn write_to_primary(&self, _item: ClipboardItem) {}

    #[cfg(target_os = "macos")]
    fn read_from_find_pasteboard(&self) -> Option<ClipboardItem> {
        None
    }

    #[cfg(target_os = "macos")]
    fn write_to_find_pasteboard(&self, _item: ClipboardItem) {}

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn keyboard_layout(&self) -> Box<dyn crate::PlatformKeyboardLayout> {
        Box::new(StreamingKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn crate::PlatformKeyboardMapper> {
        Rc::new(crate::DummyKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {}
}

use crate::{Bounds, DisplayId, Pixels, Point, Size};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug)]
struct StreamingDisplay {
    bounds: Bounds<Pixels>,
}

impl StreamingDisplay {
    fn new(config: &StreamingConfig) -> Self {
        Self {
            bounds: Bounds {
                origin: Point::default(),
                size: Size {
                    width: Pixels(config.width),
                    height: Pixels(config.height),
                },
            },
        }
    }
}

impl PlatformDisplay for StreamingDisplay {
    fn id(&self) -> DisplayId {
        DisplayId(1)
    }

    fn uuid(&self) -> Result<Uuid> {
        Ok(Uuid::new_v4())
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }
}

struct StreamingKeyboardLayout;

impl crate::PlatformKeyboardLayout for StreamingKeyboardLayout {
    fn id(&self) -> &str {
        "us"
    }

    fn name(&self) -> &str {
        "US"
    }
}

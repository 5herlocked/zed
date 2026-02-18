use crate::{
    AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DummyKeyboardMapper,
    ForegroundExecutor, Keymap, NoopTextSystem, Platform, PlatformDisplay, PlatformKeyboardLayout,
    PlatformKeyboardMapper, PlatformTextSystem, Task, ThermalState, WebDisplay, WebWindow,
    WindowAppearance, WindowParams,
};
use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

pub(crate) struct WebPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    display: Rc<dyn PlatformDisplay>,
}

impl WebPlatform {
    pub fn new(
        background_executor: BackgroundExecutor,
        foreground_executor: ForegroundExecutor,
    ) -> Rc<Self> {
        Rc::new(Self {
            background_executor,
            foreground_executor,
            text_system: Arc::new(NoopTextSystem),
            display: Rc::new(WebDisplay::default()),
        })
    }
}

struct WebKeyboardLayout;

impl PlatformKeyboardLayout for WebKeyboardLayout {
    fn id(&self) -> &str {
        "web"
    }

    fn name(&self) -> &str {
        "Web Keyboard Layout"
    }
}

impl Platform for WebPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(WebKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, _: Box<dyn FnMut()>) {}

    fn on_thermal_state_change(&self, _: Box<dyn FnMut()>) {}

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        on_finish_launching();
    }

    fn quit(&self) {}

    fn restart(&self, _binary_path: Option<PathBuf>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![self.display.clone()]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        None
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn crate::PlatformWindow>> {
        Ok(Box::new(WebWindow::new(
            handle,
            params,
            self.display.clone(),
        )))
    }

    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn open_url(&self, _url: &str) {}

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {}

    fn prompt_for_paths(
        &self,
        _options: crate::PathPromptOptions,
    ) -> futures::channel::oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = futures::channel::oneshot::channel();
        drop(tx);
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> futures::channel::oneshot::Receiver<Result<Option<PathBuf>>> {
        let (tx, rx) = futures::channel::oneshot::channel();
        drop(tx);
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {}

    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {}

    fn set_menus(&self, _menus: Vec<crate::Menu>, _keymap: &Keymap) {}

    fn set_dock_menu(&self, _menu: Vec<crate::MenuItem>, _keymap: &Keymap) {}

    fn add_recent_document(&self, _path: &Path) {}

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn crate::Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(
        &self,
        _callback: Box<dyn FnMut(&dyn crate::Action) -> bool>,
    ) {
    }

    fn app_path(&self) -> Result<PathBuf> {
        Ok(PathBuf::from("/"))
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Ok(PathBuf::from("/"))
    }

    fn set_cursor_style(&self, _style: CursorStyle) {}

    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        None
    }

    fn write_to_clipboard(&self, _item: ClipboardItem) {}

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn register_url_scheme(&self, _: &str) -> Task<anyhow::Result<()>> {
        Task::ready(Ok(()))
    }
}

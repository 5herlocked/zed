use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, ForegroundExecutor,
    Image, KeyDownEvent, Keymap, Menu, MenuItem, PathPromptOptions, Platform, PlatformDisplay,
    PlatformTextSystem, Point, Result, ScreenCaptureSource, SemanticVersion, Size, Task,
    TouchPhase, WindowAppearance, WindowParams,
};
use anyhow::{anyhow, Context as _};
use futures::channel::oneshot;
use parking_lot::Mutex;
use std::ffi::c_void;
use std::{
    any::Any,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use objc2::ffi::id;
use objc2::msg_send;
use objc2_ui_kit::UIDevice;
#[cfg(feature = "font-kit")]
use crate::TextSystem;

use super::dispatcher::{IosPlatformDispatcher, UIBackgroundTaskIdentifier};

pub(crate) struct IosPlatform(Mutex<IosPlatformState>);

pub(crate) struct IosPlatformState {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    app_did_enter_background: Option<Box<dyn FnMut()>>,
    app_will_enter_foreground: Option<Box<dyn FnMut()>>,
    menu_command: Option<Box<dyn FnMut(&dyn Action)>>,
    validate_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    will_open_menu: Option<Box<dyn FnMut()>>,
    menu_actions: Vec<Box<dyn Action>>,
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    finish_launching: Option<Box<dyn FnOnce()>>,
    background_task_id: UIBackgroundTaskIdentifier,
    quit: Option<Box<dyn FnMut()>>,
}

impl Default for IosPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl IosPlatform {
    pub(crate) fn new() -> Self {
        let dispatcher = Arc::new(IosPlatformDispatcher::new());

        #[cfg(feature = "font-kit")]
        let text_system = Arc::new(TextSystem::new());

        #[cfg(not(feature = "font-kit"))]
        let text_system = Arc::new(crate::NoopTextSystem::new());

        Self(Mutex::new(IosPlatformState {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system,
            app_did_enter_background: None,
            app_will_enter_foreground: None,
            menu_command: None,
            validate_menu_command: None,
            will_open_menu: None,
            menu_actions: Default::default(),
            open_urls: None,
            finish_launching: None,
            background_task_id: 0,
            quit: None,
        }))
    }

    fn os_version() -> Result<SemanticVersion> {
        unsafe {
            // Extract UIDevice currentDevice systemVersion
            let device: id = UIDevice::;
            let system_version: id = msg_send![device, systemVersion];
            let version_str = system_version.to_str();

            // Parse version string, e.g. "15.0", into components
            let components: Vec<&str> = version_str.split('.').collect();
            let major = components
                .get(0)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            let minor = components
                .get(1)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            let patch = components
                .get(2)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);

            Ok(SemanticVersion::new(major, minor, patch))
        }
    }
}

impl Platform for IosPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.0.lock().background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.0.lock().foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.0.lock().text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        self.0.lock().finish_launching = Some(on_finish_launching);
        // UIKit app lifecycle is managed by the AppDelegate, which will call
        // on_finish_launching when application(_:didFinishLaunchingWithOptions:) is called
    }

    fn quit(&self) {
        // In iOS, apps are terminated by the OS, not by the app itself.
        // The app can request termination by calling exit(0), but this is not recommended.
        if let Some(mut callback) = self.0.lock().quit.take() {
            callback();
        }
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {
        // Not applicable on iOS
    }

    fn activate(&self, _ignoring_other_apps: bool) {
        // No direct equivalent in iOS
    }

    fn hide(&self) {
        // No direct equivalent in iOS
    }

    fn hide_other_apps(&self) {
        // Not applicable on iOS
    }

    fn unhide_other_apps(&self) {
        // Not applicable on iOS
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        // In iOS, the device screen is the primary (and only) display
        Some(Rc::new(super::display::IosDisplay::primary()))
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        // For iOS, only one display is typically available
        vec![Rc::new(super::display::IosDisplay::primary())]
    }

    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Box<dyn ScreenCaptureSource>>>> {
        // Screen recording is limited on iOS; would require RPScreenRecorder
        let (tx, rx) = oneshot::channel();
        let _ = tx.send(Ok(Vec::new())); // Send empty list
        rx
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        super::window::IosWindow::active_window()
    }

    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        // iOS typically has only one window active at a time
        // Return the active window as the only item in the stack
        self.active_window().map(|handle| vec![handle])
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Result<Box<dyn crate::PlatformWindow>> {
        Ok(Box::new(super::window::IosWindow::open(
            handle,
            options,
            self.foreground_executor(),
        )))
    }

    fn window_appearance(&self) -> WindowAppearance {
        unsafe {
            let trait_collection = super::window::get_current_trait_collection();
            WindowAppearance::from_native(trait_collection)
        }
    }

    fn open_url(&self, url: &str) {
        unsafe {
            let url_class = class!(NSURL);
            let url_str = super::utils::ns_string(url);
            let url_obj: id = msg_send![url_class, URLWithString: url_str];

            if url_obj != nil {
                let app = class!(UIApplication);
                let shared: id = msg_send![app, sharedApplication];
                let _: BOOL = msg_send![shared, openURL:url_obj
                                  options:class!(NSDictionary).dictionary()
                        completionHandler:nil];
            }
        }
    }

    fn register_url_scheme(&self, scheme: &str) -> Task<anyhow::Result<()>> {
        // URL scheme registration is done in Info.plist on iOS, not programmatically
        Task::ready(Err(anyhow!(
            "URL scheme registration must be done in Info.plist on iOS"
        )))
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.0.lock().open_urls = Some(callback);
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().quit = Some(callback);
    }

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {
        // Not applicable for iOS
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {
        // Would need to use UIKeyboardDidChangeFrameNotification
    }

    fn keyboard_layout(&self) -> String {
        unsafe {
            // Get current keyboard language
            let input_mode = class!(UITextInputMode);
            let current: id = msg_send![input_mode, currentInputMode];
            if current != nil {
                let primary_language: id = msg_send![current, primaryLanguage];
                if primary_language != nil {
                    return primary_language.to_str().to_string();
                }
            }
            "en-US".to_string() // Default fallback
        }
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        // Would need UIDocumentPickerViewController
        let (tx, rx) = oneshot::channel();
        let _ = tx.send(Ok(None));
        rx
    }

    fn prompt_for_new_path(&self, _directory: &Path) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        // Would need UIDocumentPickerViewController in export mode
        let (tx, rx) = oneshot::channel();
        let _ = tx.send(Ok(None));
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {
        // Not directly applicable on iOS
    }

    fn open_with_system(&self, _path: &Path) {
        // Would need UIDocumentInteractionController
    }

    fn app_path(&self) -> Result<PathBuf> {
        unsafe {
            let bundle = class!(NSBundle);
            let main_bundle: id = msg_send![bundle, mainBundle];
            let path: id = msg_send![main_bundle, bundlePath];
            if path == nil {
                Err(anyhow!("Failed to retrieve app bundle path"))
            } else {
                Ok(super::utils::path_from_objc(path))
            }
        }
    }

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {
        // iOS doesn't have traditional menus; could implement as context menus
    }

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {
        // Not applicable on iOS
    }

    fn add_recent_document(&self, _path: &Path) {
        // Could use NSUserActivity for document interactions
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        unsafe {
            let bundle = class!(NSBundle);
            let main_bundle: id = msg_send![bundle, mainBundle];
            let name_str = super::utils::ns_string(name);
            let path: id = msg_send![main_bundle, pathForAuxiliaryExecutable:name_str];

            if path == nil {
                Err(anyhow!("Auxiliary executable '{}' not found", name))
            } else {
                Ok(super::utils::path_from_objc(path))
            }
        }
    }

    fn set_cursor_style(&self, _style: CursorStyle) {
        // Not applicable on iOS (no cursor)
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        // iOS typically auto-hides scrollbar indicators
        true
    }

    fn write_to_clipboard(&self, _item: ClipboardItem) {
        // Would use UIPasteboard
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        // Would use UIPasteboard
        None
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        // Would use Keychain Services API
        Task::ready(Err(anyhow!("Not implemented")))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        // Would use Keychain Services API
        Task::ready(Err(anyhow!("Not implemented")))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        // Would use Keychain Services API
        Task::ready(Err(anyhow!("Not implemented")))
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.0.lock().menu_command = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().will_open_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.0.lock().validate_menu_command = Some(callback);
    }
}

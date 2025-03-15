use crate::{PlatformDispatcher, TaskLabel};
use async_task::Runnable;
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use std::{ffi::c_void, sync::Arc, time::Duration};
use std::ffi::{c_long, c_ulong};
use std::ptr::NonNull;
use objc2::{class, msg_send};

// UIKit background task identifier
pub type UIBackgroundTaskIdentifier = usize;
pub const INVALID_BACKGROUND_TASK: UIBackgroundTaskIdentifier = usize::MAX;

pub(crate) struct IosPlatformDispatcher {
    parker: Arc<Mutex<Parker>>,
}

impl Default for IosPlatformDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl IosPlatformDispatcher {
    pub fn new() -> Self {
        IosPlatformDispatcher {
            parker: Arc::new(Mutex::new(Parker::new())),
        }
    }
}

impl PlatformDispatcher for IosPlatformDispatcher {
    fn is_main_thread(&self) -> bool {
        unsafe {
            let is_main_thread: bool = msg_send![class!(NSThread), isMainThread];
            is_main_thread
        }
    }

    fn dispatch(&self, runnable: Runnable, _: Option<TaskLabel>) {
        unsafe {
            let queue = dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH, 0);
            dispatch_async_f(
                queue,
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        unsafe {
            dispatch_async_f(
                dispatch_get_main_queue(),
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        unsafe {
            let queue = dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH, 0);
            let when = dispatch_time(DISPATCH_TIME_NOW, duration.as_nanos() as i64);
            dispatch_after_f(
                when,
                queue,
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn park(&self, timeout: Option<Duration>) -> bool {
        if let Some(timeout) = timeout {
            self.parker.lock().park_timeout(timeout)
        } else {
            self.parker.lock().park();
            true
        }
    }

    fn unparker(&self) -> Unparker {
        self.parker.lock().unparker()
    }
}

extern "C" fn trampoline(runnable: *mut c_void) {
    let task = unsafe { Runnable::<()>::from_raw(NonNull::new_unchecked(runnable as *mut ())) };
    task.run();
}

// Import required libdispatch functions
#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_get_main_queue() -> DispatchQueueT;
    fn dispatch_get_global_queue(priority: c_long, flags: c_ulong) -> DispatchQueueT;
    fn dispatch_async_f(
        queue: DispatchQueueT,
        context: *mut c_void,
        work: Option<extern "C" fn(*mut c_void)>,
    );
    fn dispatch_after_f(
        when: DispatchTimeT,
        queue: DispatchQueueT,
        context: *mut c_void,
        work: Option<extern "C" fn(*mut c_void)>,
    );
    fn dispatch_time(when: DispatchTimeT, delta: i64) -> DispatchTimeT;
}

type DispatchQueueT = *mut c_void;
type DispatchTimeT = u64;

const DISPATCH_TIME_NOW: DispatchTimeT = 0;
const DISPATCH_QUEUE_PRIORITY_HIGH: c_long = 2;

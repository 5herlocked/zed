use crate::{PlatformDispatcher, Priority, RunnableVariant, TaskTiming, ThreadTaskTimings};
use std::time::{Duration, Instant};

pub(crate) struct WebDispatcher;

impl PlatformDispatcher for WebDispatcher {
    fn get_all_timings(&self) -> Vec<ThreadTaskTimings> {
        Vec::new()
    }

    fn get_current_thread_timings(&self) -> Vec<TaskTiming> {
        Vec::new()
    }

    fn is_main_thread(&self) -> bool {
        true
    }

    fn dispatch(&self, runnable: RunnableVariant, _priority: Priority) {
        runnable.run();
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        runnable.run();
    }

    fn dispatch_after(&self, _duration: Duration, runnable: RunnableVariant) {
        // In a real web implementation, this would use setTimeout.
        // For the stub, run immediately.
        runnable.run();
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        f();
    }
}

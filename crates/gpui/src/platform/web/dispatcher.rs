use crate::{PlatformDispatcher, Priority, RunnableVariant, TaskTiming, ThreadTaskTimings};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

struct DispatchQueue {
    runnables: VecDeque<RunnableVariant>,
    is_scheduled: bool,
}

pub(crate) struct WebDispatcher {
    queue: Rc<RefCell<DispatchQueue>>,
}

impl WebDispatcher {
    pub fn new() -> Self {
        Self {
            queue: Rc::new(RefCell::new(DispatchQueue {
                runnables: VecDeque::new(),
                is_scheduled: false,
            })),
        }
    }

    fn schedule_drain(&self) {
        let mut queue = self.queue.borrow_mut();
        if queue.is_scheduled {
            return;
        }
        queue.is_scheduled = true;
        let queue_handle = self.queue.clone();
        let closure = Closure::once(move || {
            drain_queue(&queue_handle);
        });
        let global = js_sys::global().unchecked_into::<web_sys::Window>();
        global
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                0,
            )
            .ok();
        closure.forget();
    }
}

fn drain_queue(queue: &Rc<RefCell<DispatchQueue>>) {
    loop {
        let runnable = {
            let mut borrowed = queue.borrow_mut();
            let runnable = borrowed.runnables.pop_front();
            if runnable.is_none() {
                borrowed.is_scheduled = false;
            }
            runnable
        };
        match runnable {
            Some(runnable) => {
                runnable.run();
            }
            None => break,
        }
    }
}

// SAFETY: In WASM there is only one thread, so Send + Sync are trivially satisfied.
unsafe impl Send for WebDispatcher {}
unsafe impl Sync for WebDispatcher {}

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
        self.queue.borrow_mut().runnables.push_back(runnable);
        self.schedule_drain();
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        self.queue.borrow_mut().runnables.push_back(runnable);
        self.schedule_drain();
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let millis = duration.as_millis() as i32;
        let closure = Closure::once(move || {
            runnable.run();
        });
        let global = js_sys::global().unchecked_into::<web_sys::Window>();
        global
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                millis,
            )
            .ok();
        closure.forget();
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        f();
    }
}

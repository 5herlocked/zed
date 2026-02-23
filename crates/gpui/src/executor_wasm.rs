use crate::{App, PlatformDispatcher};
use crate::wasm_shims::RunnableMeta;
use futures::channel::mpsc;
use futures::FutureExt;
use std::{
    fmt::Debug,
    future::Future,
    marker::PhantomData,
    mem,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use crate::time_compat::Instant;

pub use crate::wasm_shims::Priority;

/// Re-export for compatibility.
pub type SchedulerForegroundExecutor = ForegroundExecutor;

/// A pointer to the executor that is currently running,
/// for spawning background tasks.
#[derive(Clone)]
pub struct BackgroundExecutor {
    dispatcher: Arc<dyn PlatformDispatcher>,
}

/// A pointer to the executor that is currently running,
/// for spawning tasks on the main thread.
#[derive(Clone)]
pub struct ForegroundExecutor {
    dispatcher: Arc<dyn PlatformDispatcher>,
    not_send: PhantomData<Rc<()>>,
}

/// Task is a primitive that allows work to happen in the background.
#[must_use]
pub struct Task<T>(TaskInner<T>);

enum TaskInner<T> {
    Ready(Option<T>),
    Spawned(futures::channel::oneshot::Receiver<T>),
}

impl<T> Debug for Task<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            TaskInner::Ready(_) => f.write_str("Task::Ready"),
            TaskInner::Spawned(_) => f.write_str("Task::Spawned"),
        }
    }
}

/// A task that returns None if cancelled.
pub struct FallibleTask<T>(Task<T>);

impl<T> Task<T> {
    /// Creates a new task that will resolve with the value.
    pub fn ready(val: T) -> Self {
        Task(TaskInner::Ready(Some(val)))
    }

    /// Returns true if the task has completed or was created with `Task::ready`.
    pub fn is_ready(&self) -> bool {
        matches!(&self.0, TaskInner::Ready(_))
    }

    /// Detaching a task runs it to completion in the background.
    pub fn detach(self) {
        // On WASM, tasks are already running. Just forget the handle.
        mem::forget(self);
    }

    /// Converts this task into a fallible task.
    pub fn fallible(self) -> FallibleTask<T> {
        FallibleTask(self)
    }
}

impl<T, E> Task<Result<T, E>>
where
    T: 'static,
    E: 'static + Debug,
{
    /// Run the task to completion in the background and log any errors that occur.
    #[track_caller]
    pub fn detach_and_log_err(self, _cx: &App) {
        self.detach();
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match &mut this.0 {
            TaskInner::Ready(val) => {
                if let Some(val) = val.take() {
                    Poll::Ready(val)
                } else {
                    Poll::Pending
                }
            }
            TaskInner::Spawned(rx) => {
                let pinned = unsafe { Pin::new_unchecked(rx) };
                match pinned.poll(cx) {
                    Poll::Ready(Ok(val)) => Poll::Ready(val),
                    Poll::Ready(Err(_)) => panic!("task was cancelled"),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}

impl<T> Future for FallibleTask<T> {
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match &mut (this.0).0 {
            TaskInner::Ready(val) => Poll::Ready(val.take()),
            TaskInner::Spawned(rx) => {
                let pinned = unsafe { Pin::new_unchecked(rx) };
                match pinned.poll(cx) {
                    Poll::Ready(Ok(val)) => Poll::Ready(Some(val)),
                    Poll::Ready(Err(_)) => Poll::Ready(None),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}

impl BackgroundExecutor {
    /// Creates a new BackgroundExecutor from the given PlatformDispatcher.
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self { dispatcher }
    }

    /// Close this executor.
    pub fn close(&self) {}

    /// Enqueues the given future to be run to completion on a background thread.
    #[track_caller]
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        let (tx, rx) = futures::channel::oneshot::channel();
        let dispatcher = self.dispatcher.clone();
        let location = std::panic::Location::caller();
        let closed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let runnable_meta = RunnableMeta { location, closed };
        let (runnable, task) = async_task::Builder::new()
            .metadata(runnable_meta)
            .spawn(
                move |_| async move {
                    let result = future.await;
                    let _ = tx.send(result);
                },
                move |runnable| {
                    dispatcher.dispatch(runnable, Priority::default());
                },
            );
        task.detach();
        runnable.schedule();
        Task(TaskInner::Spawned(rx))
    }

    /// Enqueues the given future with priority.
    #[track_caller]
    pub fn spawn_with_priority<R>(
        &self,
        _priority: Priority,
        future: impl Future<Output = R> + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
    {
        self.spawn(future)
    }

    /// Get the current time.
    pub fn now(&self) -> Instant {
        self.dispatcher.now()
    }

    /// Returns a task that will complete after the given duration.
    pub fn timer(&self, duration: Duration) -> Task<()> {
        if duration.is_zero() {
            return Task::ready(());
        }
        let (tx, rx) = futures::channel::oneshot::channel();
        let dispatcher = self.dispatcher.clone();
        let location = std::panic::Location::caller();
        let closed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let runnable_meta = RunnableMeta { location, closed };
        let (runnable, _task) = async_task::Builder::new()
            .metadata(runnable_meta)
            .spawn(
                move |_| async move {
                    let _ = tx.send(());
                },
                move |runnable| {
                    dispatcher.dispatch_after(duration, runnable);
                },
            );
        runnable.schedule();
        Task(TaskInner::Spawned(rx))
    }

    /// How many CPUs are available.
    pub fn num_cpus(&self) -> usize {
        1
    }

    /// Whether we're on the main thread.
    pub fn is_main_thread(&self) -> bool {
        self.dispatcher.is_main_thread()
    }

    #[doc(hidden)]
    pub fn dispatcher(&self) -> &Arc<dyn PlatformDispatcher> {
        &self.dispatcher
    }

    /// Scoped lets you start a number of tasks and waits
    /// for all of them to complete before returning.
    pub async fn scoped<'scope, F>(&self, scheduler: F)
    where
        F: FnOnce(&mut Scope<'scope>),
    {
        let mut scope = Scope::new(self.clone(), Priority::default());
        (scheduler)(&mut scope);
        let spawned = mem::take(&mut scope.futures)
            .into_iter()
            .map(|f| self.spawn(f))
            .collect::<Vec<_>>();
        for task in spawned {
            task.await;
        }
    }

    /// Scoped with priority.
    pub async fn scoped_priority<'scope, F>(&self, _priority: Priority, scheduler: F)
    where
        F: FnOnce(&mut Scope<'scope>),
    {
        self.scoped(scheduler).await;
    }

    /// Await on background.
    pub async fn await_on_background<R>(&self, future: impl Future<Output = R> + Send) -> R
    where
        R: Send,
    {
        future.await
    }
}

impl ForegroundExecutor {
    /// Creates a new ForegroundExecutor from the given PlatformDispatcher.
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self {
            dispatcher,
            not_send: PhantomData,
        }
    }

    /// Close this executor.
    pub fn close(&self) {}

    /// Enqueues the given Task to run on the main thread.
    #[track_caller]
    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where
        R: 'static,
    {
        let (tx, rx) = futures::channel::oneshot::channel();
        let dispatcher = self.dispatcher.clone();
        let location = std::panic::Location::caller();
        let closed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let runnable_meta = RunnableMeta { location, closed };
        let (runnable, task) = async_task::Builder::new()
            .metadata(runnable_meta)
            .spawn_local(
                move |_| async move {
                    let result = future.await;
                    let _ = tx.send(result);
                },
                move |runnable| {
                    dispatcher.dispatch_on_main_thread(runnable, Priority::default());
                },
            );
        // Keep the async_task handle alive so the task isn't cancelled.
        task.detach();
        runnable.schedule();
        Task(TaskInner::Spawned(rx))
    }

    /// Enqueues the given Task to run on the main thread with the given priority.
    #[track_caller]
    pub fn spawn_with_priority<R>(
        &self,
        _priority: Priority,
        future: impl Future<Output = R> + 'static,
    ) -> Task<R>
    where
        R: 'static,
    {
        self.spawn(future)
    }

    /// Block the current thread until the given future resolves.
    pub fn block_on<R>(&self, future: impl Future<Output = R>) -> R {
        // On WASM, we can't truly block. This is a best-effort stub.
        // In practice, WASM code should use async patterns.
        panic!("block_on is not supported on WASM");
    }

    /// Block with timeout.
    pub fn block_with_timeout<R, Fut: Future<Output = R>>(
        &self,
        _duration: Duration,
        _future: Fut,
    ) -> Result<R, Fut> {
        panic!("block_with_timeout is not supported on WASM");
    }

    #[doc(hidden)]
    pub fn dispatcher(&self) -> &Arc<dyn PlatformDispatcher> {
        &self.dispatcher
    }

    #[doc(hidden)]
    pub fn scheduler_executor(&self) -> ForegroundExecutor {
        self.clone()
    }
}

/// Scope manages a set of tasks that are enqueued and waited on together.
pub struct Scope<'a> {
    executor: BackgroundExecutor,
    priority: Priority,
    futures: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    tx: Option<mpsc::Sender<()>>,
    rx: mpsc::Receiver<()>,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Scope<'a> {
    fn new(executor: BackgroundExecutor, priority: Priority) -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self {
            executor,
            priority,
            tx: Some(tx),
            rx,
            futures: Default::default(),
            lifetime: PhantomData,
        }
    }

    /// How many CPUs are available to the dispatcher.
    pub fn num_cpus(&self) -> usize {
        self.executor.num_cpus()
    }

    /// Spawn a future into this scope.
    #[track_caller]
    pub fn spawn<F>(&mut self, f: F)
    where
        F: Future<Output = ()> + Send + 'a,
    {
        let tx = self.tx.clone().unwrap();
        let f = unsafe {
            mem::transmute::<
                Pin<Box<dyn Future<Output = ()> + Send + 'a>>,
                Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
            >(Box::pin(async move {
                f.await;
                drop(tx);
            }))
        };
        self.futures.push(f);
    }
}

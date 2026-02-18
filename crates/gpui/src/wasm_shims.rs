// Minimal stubs for types/traits that GPUI core code needs from `util` and `scheduler`
// when compiling to wasm32-unknown-unknown, where those crates cannot compile.

use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::{self, Debug},
    future::Future,
    hash::{Hash, Hasher},
    ops::AddAssign,
    panic::Location,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering as AtomicOrdering},
    },
    task::{Context, Poll},
    time::Duration,
};

// --- debug_panic macro ---

macro_rules! debug_panic {
    ( $($fmt_arg:tt)* ) => {
        if cfg!(debug_assertions) {
            panic!( $($fmt_arg)* );
        } else {
            log::error!("{}", format_args!($($fmt_arg)*));
        }
    };
}

pub(crate) use debug_panic;

// --- ResultExt ---

pub trait ResultExt<E> {
    type Ok;

    fn log_err(self) -> Option<Self::Ok>;
    fn debug_assert_ok(self, reason: &str) -> Self;
    fn warn_on_err(self) -> Option<Self::Ok>;
    fn log_with_level(self, level: log::Level) -> Option<Self::Ok>;
    fn anyhow(self) -> anyhow::Result<Self::Ok>
    where
        E: Into<anyhow::Error>;
}

impl<T, E> ResultExt<E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    #[track_caller]
    fn log_err(self) -> Option<T> {
        self.log_with_level(log::Level::Error)
    }

    #[track_caller]
    fn debug_assert_ok(self, reason: &str) -> Self {
        if let Err(error) = &self {
            debug_panic!("{reason} - {error:?}");
        }
        self
    }

    #[track_caller]
    fn warn_on_err(self) -> Option<T> {
        self.log_with_level(log::Level::Warn)
    }

    #[track_caller]
    fn log_with_level(self, level: log::Level) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                let caller = Location::caller();
                log::log!(level, "{}:{}: {error:?}", caller.file(), caller.line());
                None
            }
        }
    }

    fn anyhow(self) -> anyhow::Result<T>
    where
        E: Into<anyhow::Error>,
    {
        self.map_err(Into::into)
    }
}

// --- TryFutureExt ---

pub trait TryFutureExt {
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn log_tracked_err(self, location: std::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;
}

impl<F, T, E> TryFutureExt for F
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    #[track_caller]
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = *Location::caller();
        LogErrorFuture(self, log::Level::Error, location)
    }

    fn log_tracked_err(self, location: std::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        LogErrorFuture(self, log::Level::Error, location)
    }

    #[track_caller]
    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = *Location::caller();
        LogErrorFuture(self, log::Level::Warn, location)
    }
}

#[pin_project::pin_project]
pub struct LogErrorFuture<F>(#[pin] F, log::Level, std::panic::Location<'static>);

impl<F, T, E> Future for LogErrorFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.0.poll(cx) {
            Poll::Ready(Ok(value)) => Poll::Ready(Some(value)),
            Poll::Ready(Err(error)) => {
                log::log!(*this.1, "{}:{}: {error:?}", this.2.file(), this.2.line());
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

// --- Deferred ---

pub struct Deferred<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Deferred<F> {
    pub fn new(f: F) -> Self {
        Self(Some(f))
    }

    pub fn abort(mut self) {
        self.0.take();
    }
}

impl<F: FnOnce()> Drop for Deferred<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f()
        }
    }
}

// --- defer ---

pub fn defer<F: FnOnce()>(f: F) -> Deferred<F> {
    Deferred::new(f)
}

// --- post_inc ---

pub fn post_inc<T: From<u8> + AddAssign<T> + Copy>(value: &mut T) -> T {
    let prev = *value;
    *value += T::from(1);
    prev
}

// --- measure ---

pub fn measure<R>(label: &str, f: impl FnOnce() -> R) -> R {
    log::debug!("measure: {label} (start)");
    let result = f();
    log::debug!("measure: {label} (end)");
    result
}

// --- ArcCow ---

/// Arc-based copy-on-write type for WASM.
pub mod arc_cow {
    use super::*;

    /// A copy-on-write smart pointer that can hold either a borrowed reference or an Arc.
    pub enum ArcCow<'a, T: ?Sized> {
        /// A borrowed reference.
        Borrowed(&'a T),
        /// An owned Arc.
        Owned(Arc<T>),
    }

    impl<T: ?Sized + PartialEq> PartialEq for ArcCow<'_, T> {
        fn eq(&self, other: &Self) -> bool {
            self.as_ref() == other.as_ref()
        }
    }

    impl<T: ?Sized + PartialOrd> PartialOrd for ArcCow<'_, T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.as_ref().partial_cmp(other.as_ref())
        }
    }

    impl<T: ?Sized + Ord> Ord for ArcCow<'_, T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_ref().cmp(other.as_ref())
        }
    }

    impl<T: ?Sized + Eq> Eq for ArcCow<'_, T> {}

    impl<T: ?Sized + Hash> Hash for ArcCow<'_, T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            match self {
                Self::Borrowed(borrowed) => Hash::hash(borrowed, state),
                Self::Owned(owned) => Hash::hash(&**owned, state),
            }
        }
    }

    impl<T: ?Sized> Clone for ArcCow<'_, T> {
        fn clone(&self) -> Self {
            match self {
                Self::Borrowed(borrowed) => Self::Borrowed(borrowed),
                Self::Owned(owned) => Self::Owned(owned.clone()),
            }
        }
    }

    impl<'a, T: ?Sized> From<&'a T> for ArcCow<'a, T> {
        fn from(s: &'a T) -> Self {
            Self::Borrowed(s)
        }
    }

    impl<T: ?Sized> From<Arc<T>> for ArcCow<'_, T> {
        fn from(s: Arc<T>) -> Self {
            Self::Owned(s)
        }
    }

    impl<T: ?Sized> From<&'_ Arc<T>> for ArcCow<'_, T> {
        fn from(s: &'_ Arc<T>) -> Self {
            Self::Owned(s.clone())
        }
    }

    impl From<String> for ArcCow<'_, str> {
        fn from(value: String) -> Self {
            Self::Owned(value.into())
        }
    }

    impl From<&String> for ArcCow<'_, str> {
        fn from(value: &String) -> Self {
            Self::Owned(value.clone().into())
        }
    }

    impl<'a> From<Cow<'a, str>> for ArcCow<'a, str> {
        fn from(value: Cow<'a, str>) -> Self {
            match value {
                Cow::Borrowed(borrowed) => Self::Borrowed(borrowed),
                Cow::Owned(owned) => Self::Owned(owned.into()),
            }
        }
    }

    impl<T> From<Vec<T>> for ArcCow<'_, [T]> {
        fn from(vec: Vec<T>) -> Self {
            ArcCow::Owned(Arc::from(vec))
        }
    }

    impl<'a> From<&'a str> for ArcCow<'a, [u8]> {
        fn from(s: &'a str) -> Self {
            ArcCow::Borrowed(s.as_bytes())
        }
    }

    impl<T: ?Sized + std::borrow::ToOwned> std::borrow::Borrow<T> for ArcCow<'_, T> {
        fn borrow(&self) -> &T {
            match self {
                ArcCow::Borrowed(borrowed) => borrowed,
                ArcCow::Owned(owned) => owned.as_ref(),
            }
        }
    }

    impl<T: ?Sized> std::ops::Deref for ArcCow<'_, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            match self {
                ArcCow::Borrowed(s) => s,
                ArcCow::Owned(s) => s.as_ref(),
            }
        }
    }

    impl<T: ?Sized> AsRef<T> for ArcCow<'_, T> {
        fn as_ref(&self) -> &T {
            match self {
                ArcCow::Borrowed(borrowed) => borrowed,
                ArcCow::Owned(owned) => owned.as_ref(),
            }
        }
    }

    impl<T: ?Sized + Debug> Debug for ArcCow<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ArcCow::Borrowed(borrowed) => Debug::fmt(borrowed, f),
                ArcCow::Owned(owned) => Debug::fmt(&**owned, f),
            }
        }
    }
}

// --- Priority (from scheduler) ---

/// Task priority levels for the WASM executor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Priority {
    /// Realtime audio priority.
    RealtimeAudio,
    /// High priority.
    High,
    /// Medium priority (default).
    #[default]
    Medium,
    /// Low priority.
    Low,
}

impl Priority {
    /// Returns the relative probability weight for this priority level.
    pub const fn weight(self) -> u32 {
        match self {
            Priority::High => 60,
            Priority::Medium => 30,
            Priority::Low => 10,
            Priority::RealtimeAudio => 100,
        }
    }
}

/// Metadata attached to each runnable task.
pub struct RunnableMeta {
    /// The source location where the task was spawned.
    pub location: &'static Location<'static>,
    /// Shared flag indicating whether the scheduler has been closed.
    pub closed: Arc<AtomicBool>,
}

impl RunnableMeta {
    /// Returns true if the scheduler has been closed.
    pub fn is_closed(&self) -> bool {
        self.closed.load(AtomicOrdering::SeqCst)
    }
}

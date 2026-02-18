//! Streaming platform implementation for headless-web mode.
//!
//! Provides a `StreamingWindow` that satisfies `PlatformWindow` without a GPU
//! or display server. Each frame, GPUI's render pipeline captures a
//! `DisplayTree` and makes it available to the transport layer via a channel.

mod window;

pub use window::StreamingWindow;

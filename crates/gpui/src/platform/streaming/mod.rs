//! Streaming platform implementation for headless-web mode.
//!
//! Provides a `StreamingPlatform` (full Platform impl) and `StreamingWindow`
//! (PlatformWindow impl) that run GPUI without a GPU or display server.
//! Each frame, GPUI's render pipeline captures a `DisplayTree` and makes
//! it available to the transport layer via a channel.

mod platform;
mod window;

pub use platform::{StreamingConfig, StreamingPlatform};
pub use window::StreamingWindow;

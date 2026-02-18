//! Web streaming platform backend for GPUI.
//!
//! This module provides the infrastructure for streaming GPUI's rendered
//! scenes to a browser-based renderer over WebSocket. It enables Zed to
//! run on a headless GPU-less Linux machine while being fully interactive
//! through a web browser. On macOS, it can run alongside the native renderer
//! to stream scenes for development and testing.
//!
//! Cross-platform modules:
//!
//! - `scene_message` defines serializable wire-format types that mirror
//!   GPUI's scene primitives. Each type has a `From` conversion from its
//!   GPUI counterpart, producing a JSON-serializable representation that
//!   the browser-side TypeScript renderer can consume.
//!
//! - `server` provides the WebSocket server for streaming frames to
//!   connected browser clients (requires the `web-streaming` feature).
//!
//! - `launch` provides `init_web_streaming`, the public entry point that
//!   wires everything together when `ZED_WEB_STREAMING=1` is set.
//!
//! Linux-only modules:
//!
//! - `StreamingAtlas` implements `PlatformAtlas` using CPU memory instead
//!   of GPU textures for headless operation.
//!
//! - `StreamingWindow` implements `PlatformWindow` and captures the `Scene`
//!   at draw time for serialization.
//!
//! - `WebStreamingClient` implements `LinuxClient` and extends the headless
//!   Linux platform with window support.
//!
//! See `docs/src/development/web-streaming-renderer.md` for the full plan.

// Cross-platform modules
mod binary_frame;
mod launch;
mod mirroring_atlas;
mod proto_encoding;
mod scene_message;
mod server;

// Linux-only modules (headless rendering support)
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod client;
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod streaming_atlas;
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod streaming_window;

// Cross-platform exports
pub use binary_frame::*;
pub use launch::*;
pub use mirroring_atlas::*;
pub use proto_encoding::*;
pub use scene_message::*;
pub use server::*;

// Linux-only exports
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub use client::*;
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub use streaming_atlas::*;
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub use streaming_window::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod auth;
#[cfg(not(target_arch = "wasm32"))]
mod conn;
#[cfg(not(target_arch = "wasm32"))]
mod message_stream;
mod notification;
#[cfg(not(target_arch = "wasm32"))]
mod peer;

#[cfg(not(target_arch = "wasm32"))]
pub use conn::Connection;
pub use notification::*;
#[cfg(not(target_arch = "wasm32"))]
pub use peer::*;
pub use proto;
pub use proto::{Receipt, TypedEnvelope, error::*};
#[cfg(not(target_arch = "wasm32"))]
mod macros;

#[cfg(feature = "gpui")]
mod proto_client;
#[cfg(feature = "gpui")]
pub use proto_client::*;

pub const PROTOCOL_VERSION: u32 = 68;

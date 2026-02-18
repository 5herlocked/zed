#[cfg(not(target_arch = "wasm32"))]
#[path = "executor_native.rs"]
mod inner;

#[cfg(target_arch = "wasm32")]
#[path = "executor_wasm.rs"]
mod inner;

pub use inner::*;

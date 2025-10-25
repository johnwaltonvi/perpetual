#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{ThreadPool, ThreadPoolBuilder};
#[cfg(target_arch = "wasm32")]
pub use wasm::{ThreadPool, ThreadPoolBuilder};

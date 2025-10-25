#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{resolve_memory_budget, MemoryBudget};
#[cfg(target_arch = "wasm32")]
pub use wasm::{resolve_memory_budget, MemoryBudget};

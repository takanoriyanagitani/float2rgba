//! Float converters using an external crate(colorgrad).

#[cfg(feature = "ext_colorgrad_wasm")]
pub mod wasm;

#[cfg(feature = "ext_colorgrad_turbo_wasm")]
pub mod turbo;

#[cfg(feature = "ext_colorgrad_rainbow_wasm")]
pub mod rainbow;

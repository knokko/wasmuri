#[cfg(not(target_arch="wasm32"))]
mod desktop;
#[cfg(not(target_arch="wasm32"))]
pub use desktop::*;

#[cfg(target_arch="wasm32")]
mod web;
#[cfg(target_arch="wasm32")]
pub use web::*;

use wasm_bindgen::prelude::*;

// Import modules
pub mod core;
pub mod filters;
pub mod utils;
pub mod wasm;

// Set up panic hook for better debugging in WebAssembly
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

// Re-export main WebAssembly interface
pub use wasm::*;
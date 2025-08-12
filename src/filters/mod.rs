pub mod color;
pub mod artistic;
pub mod correction;
pub mod transform;

use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;

pub type ImageData = ImageBuffer<Rgba<u8>, Vec<u8>>;

// Base trait for all image filters
pub trait ImageFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue>;
    fn name(&self) -> &'static str;
}

// Filter chain for applying multiple filters in sequence
pub struct FilterChain {
    filters: Vec<Box<dyn ImageFilter>>,
}

impl FilterChain {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add_filter(mut self, filter: Box<dyn ImageFilter>) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        for filter in &self.filters {
            filter.apply(image)?;
        }
        Ok(())
    }
}

// Re-export all filter modules
pub use color::*;
pub use artistic::*;
pub use correction::*;
pub use transform::*;
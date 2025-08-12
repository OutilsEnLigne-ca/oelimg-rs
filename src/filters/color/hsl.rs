use crate::filters::{ImageFilter, ImageData};
use crate::core::color_space::{Rgb, Hsl};
use crate::utils::clamp_f32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HslFilter {
    hue_shift: f32,      // -180.0 to 180.0 degrees
    saturation: f32,     // 0.0 to 2.0 (1.0 = no change)
    lightness: f32,      // 0.0 to 2.0 (1.0 = no change)
}

#[wasm_bindgen]
impl HslFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(hue_shift: f32, saturation: f32, lightness: f32) -> Self {
        Self {
            hue_shift: clamp_f32(hue_shift, -180.0, 180.0),
            saturation: clamp_f32(saturation, 0.0, 2.0),
            lightness: clamp_f32(lightness, 0.0, 2.0),
        }
    }

    // Getters for WebAssembly
    #[wasm_bindgen(getter)]
    pub fn hue_shift(&self) -> f32 {
        self.hue_shift
    }

    #[wasm_bindgen(getter)]
    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    #[wasm_bindgen(getter)]
    pub fn lightness(&self) -> f32 {
        self.lightness
    }

    // Setters for WebAssembly
    #[wasm_bindgen(setter)]
    pub fn set_hue_shift(&mut self, value: f32) {
        self.hue_shift = clamp_f32(value, -180.0, 180.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_saturation(&mut self, value: f32) {
        self.saturation = clamp_f32(value, 0.0, 2.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_lightness(&mut self, value: f32) {
        self.lightness = clamp_f32(value, 0.0, 2.0);
    }
}

impl ImageFilter for HslFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        for pixel in image.pixels_mut() {
            let rgb = Rgb::from_u8(pixel[0], pixel[1], pixel[2]);
            let mut hsl = rgb.to_hsl();
            
            // Apply adjustments
            hsl = hsl.adjust_hue(self.hue_shift);
            hsl = hsl.adjust_saturation(self.saturation);
            hsl = hsl.adjust_lightness(self.lightness);
            
            let adjusted_rgb = hsl.to_rgb();
            let (r, g, b) = adjusted_rgb.to_u8();
            
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
            // Alpha channel remains unchanged
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "HSL Adjustment"
    }
}

// Convenience functions for individual adjustments
#[wasm_bindgen]
pub struct HueFilter {
    degrees: f32,
}

#[wasm_bindgen]
impl HueFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(degrees: f32) -> Self {
        Self {
            degrees: clamp_f32(degrees, -180.0, 180.0),
        }
    }
}

impl ImageFilter for HueFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let filter = HslFilter::new(self.degrees, 1.0, 1.0);
        filter.apply(image)
    }

    fn name(&self) -> &'static str {
        "Hue Adjustment"
    }
}

#[wasm_bindgen]
pub struct SaturationFilter {
    factor: f32,
}

#[wasm_bindgen]
impl SaturationFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(factor: f32) -> Self {
        Self {
            factor: clamp_f32(factor, 0.0, 2.0),
        }
    }
}

impl ImageFilter for SaturationFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let filter = HslFilter::new(0.0, self.factor, 1.0);
        filter.apply(image)
    }

    fn name(&self) -> &'static str {
        "Saturation Adjustment"
    }
}

#[wasm_bindgen]
pub struct LightnessFilter {
    factor: f32,
}

#[wasm_bindgen]
impl LightnessFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(factor: f32) -> Self {
        Self {
            factor: clamp_f32(factor, 0.0, 2.0),
        }
    }
}

impl ImageFilter for LightnessFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let filter = HslFilter::new(0.0, 1.0, self.factor);
        filter.apply(image)
    }

    fn name(&self) -> &'static str {
        "Lightness Adjustment"
    }
}
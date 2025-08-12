use crate::filters::{ImageFilter, ImageData};
use crate::core::color_space::Rgb;
use crate::utils::clamp_f32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BrightnessFilter {
    brightness: f32, // -1.0 to 1.0 (0.0 = no change)
}

#[wasm_bindgen]
impl BrightnessFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(brightness: f32) -> Self {
        Self {
            brightness: clamp_f32(brightness, -1.0, 1.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    #[wasm_bindgen(setter)]
    pub fn set_brightness(&mut self, value: f32) {
        self.brightness = clamp_f32(value, -1.0, 1.0);
    }
}

impl ImageFilter for BrightnessFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        for pixel in image.pixels_mut() {
            let rgb = Rgb::from_u8(pixel[0], pixel[1], pixel[2]);
            
            // Apply brightness adjustment (additive)
            let adjusted = Rgb::new(
                clamp_f32(rgb.r + self.brightness, 0.0, 1.0),
                clamp_f32(rgb.g + self.brightness, 0.0, 1.0),
                clamp_f32(rgb.b + self.brightness, 0.0, 1.0),
            );
            
            let (r, g, b) = adjusted.to_u8();
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Brightness"
    }
}

#[wasm_bindgen]
pub struct ContrastFilter {
    contrast: f32, // 0.0 to 2.0 (1.0 = no change)
}

#[wasm_bindgen]
impl ContrastFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(contrast: f32) -> Self {
        Self {
            contrast: clamp_f32(contrast, 0.0, 2.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn contrast(&self) -> f32 {
        self.contrast
    }

    #[wasm_bindgen(setter)]
    pub fn set_contrast(&mut self, value: f32) {
        self.contrast = clamp_f32(value, 0.0, 2.0);
    }
}

impl ImageFilter for ContrastFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        for pixel in image.pixels_mut() {
            let rgb = Rgb::from_u8(pixel[0], pixel[1], pixel[2]);
            let adjusted = rgb.adjust_contrast(self.contrast);
            
            let (r, g, b) = adjusted.to_u8();
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Contrast"
    }
}

#[wasm_bindgen]
pub struct BrightnessContrastFilter {
    brightness: f32,
    contrast: f32,
}

#[wasm_bindgen]
impl BrightnessContrastFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(brightness: f32, contrast: f32) -> Self {
        Self {
            brightness: clamp_f32(brightness, -1.0, 1.0),
            contrast: clamp_f32(contrast, 0.0, 2.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    #[wasm_bindgen(getter)]
    pub fn contrast(&self) -> f32 {
        self.contrast
    }

    #[wasm_bindgen(setter)]
    pub fn set_brightness(&mut self, value: f32) {
        self.brightness = clamp_f32(value, -1.0, 1.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_contrast(&mut self, value: f32) {
        self.contrast = clamp_f32(value, 0.0, 2.0);
    }
}

impl ImageFilter for BrightnessContrastFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        for pixel in image.pixels_mut() {
            let rgb = Rgb::from_u8(pixel[0], pixel[1], pixel[2]);
            
            // Apply brightness first, then contrast
            let brightened = Rgb::new(
                clamp_f32(rgb.r + self.brightness, 0.0, 1.0),
                clamp_f32(rgb.g + self.brightness, 0.0, 1.0),
                clamp_f32(rgb.b + self.brightness, 0.0, 1.0),
            );
            
            let adjusted = brightened.adjust_contrast(self.contrast);
            
            let (r, g, b) = adjusted.to_u8();
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Brightness & Contrast"
    }
}
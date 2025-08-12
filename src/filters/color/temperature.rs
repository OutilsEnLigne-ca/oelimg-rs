use crate::filters::{ImageFilter, ImageData};
use crate::core::color_space::{Rgb, adjust_color_temperature};
use crate::utils::clamp_f32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ColorTemperatureFilter {
    kelvin: f32,    // 1000.0 to 40000.0 (6500.0 = daylight)
    strength: f32,  // 0.0 to 1.0 (how strong the effect is)
}

#[wasm_bindgen]
impl ColorTemperatureFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(kelvin: f32, strength: Option<f32>) -> Self {
        Self {
            kelvin: clamp_f32(kelvin, 1000.0, 40000.0),
            strength: clamp_f32(strength.unwrap_or(1.0), 0.0, 1.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn kelvin(&self) -> f32 {
        self.kelvin
    }

    #[wasm_bindgen(getter)]
    pub fn strength(&self) -> f32 {
        self.strength
    }

    #[wasm_bindgen(setter)]
    pub fn set_kelvin(&mut self, value: f32) {
        self.kelvin = clamp_f32(value, 1000.0, 40000.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_strength(&mut self, value: f32) {
        self.strength = clamp_f32(value, 0.0, 1.0);
    }

    // Preset temperature methods
    pub fn warm() -> Self {
        Self::new(3000.0, Some(0.8))
    }

    pub fn cool() -> Self {
        Self::new(9000.0, Some(0.8))
    }

    pub fn daylight() -> Self {
        Self::new(6500.0, Some(1.0))
    }

    pub fn tungsten() -> Self {
        Self::new(3200.0, Some(0.9))
    }
}

impl ImageFilter for ColorTemperatureFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        for pixel in image.pixels_mut() {
            let rgb = Rgb::from_u8(pixel[0], pixel[1], pixel[2]);
            let adjusted = adjust_color_temperature(rgb, self.kelvin);
            
            // Blend with original based on strength
            let final_rgb = Rgb::new(
                rgb.r * (1.0 - self.strength) + adjusted.r * self.strength,
                rgb.g * (1.0 - self.strength) + adjusted.g * self.strength,
                rgb.b * (1.0 - self.strength) + adjusted.b * self.strength,
            );
            
            let (r, g, b) = final_rgb.to_u8();
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Color Temperature"
    }
}

// White balance filter with tint adjustment
#[wasm_bindgen]
pub struct WhiteBalanceFilter {
    temperature: f32, // -100.0 to 100.0 (0 = no change)
    tint: f32,       // -100.0 to 100.0 (0 = no change)
}

#[wasm_bindgen]
impl WhiteBalanceFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(temperature: f32, tint: f32) -> Self {
        Self {
            temperature: clamp_f32(temperature, -100.0, 100.0),
            tint: clamp_f32(tint, -100.0, 100.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    #[wasm_bindgen(getter)]
    pub fn tint(&self) -> f32 {
        self.tint
    }

    #[wasm_bindgen(setter)]
    pub fn set_temperature(&mut self, value: f32) {
        self.temperature = clamp_f32(value, -100.0, 100.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_tint(&mut self, value: f32) {
        self.tint = clamp_f32(value, -100.0, 100.0);
    }
}

impl ImageFilter for WhiteBalanceFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        for pixel in image.pixels_mut() {
            let rgb = Rgb::from_u8(pixel[0], pixel[1], pixel[2]);
            
            // Temperature adjustment (blue-orange axis)
            let temp_factor = 1.0 + (self.temperature / 100.0) * 0.5;
            let temp_adjusted = if self.temperature > 0.0 {
                // Warmer (more yellow/red)
                Rgb::new(
                    clamp_f32(rgb.r * temp_factor, 0.0, 1.0),
                    clamp_f32(rgb.g * (1.0 + temp_factor * 0.5), 0.0, 1.0),
                    rgb.b,
                )
            } else {
                // Cooler (more blue)
                Rgb::new(
                    rgb.r,
                    clamp_f32(rgb.g * (1.0 - temp_factor * 0.5), 0.0, 1.0),
                    clamp_f32(rgb.b * (2.0 - temp_factor), 0.0, 1.0),
                )
            };
            
            // Tint adjustment (green-magenta axis)
            let tint_factor = self.tint / 100.0 * 0.3;
            let final_rgb = if self.tint > 0.0 {
                // More green
                Rgb::new(
                    clamp_f32(temp_adjusted.r - tint_factor * 0.5, 0.0, 1.0),
                    clamp_f32(temp_adjusted.g + tint_factor, 0.0, 1.0),
                    clamp_f32(temp_adjusted.b - tint_factor * 0.5, 0.0, 1.0),
                )
            } else {
                // More magenta
                Rgb::new(
                    clamp_f32(temp_adjusted.r - tint_factor * 0.5, 0.0, 1.0),
                    clamp_f32(temp_adjusted.g + tint_factor, 0.0, 1.0),
                    clamp_f32(temp_adjusted.b - tint_factor * 0.5, 0.0, 1.0),
                )
            };
            
            let (r, g, b) = final_rgb.to_u8();
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "White Balance"
    }
}
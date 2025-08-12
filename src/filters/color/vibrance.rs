use crate::filters::{ImageFilter, ImageData};
use crate::core::color_space::{Rgb, Hsl};
use crate::utils::clamp_f32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct VibranceFilter {
    vibrance: f32, // -100.0 to 100.0 (0 = no change)
}

#[wasm_bindgen]
impl VibranceFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(vibrance: f32) -> Self {
        Self {
            vibrance: clamp_f32(vibrance, -100.0, 100.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn vibrance(&self) -> f32 {
        self.vibrance
    }

    #[wasm_bindgen(setter)]
    pub fn set_vibrance(&mut self, value: f32) {
        self.vibrance = clamp_f32(value, -100.0, 100.0);
    }
}

impl ImageFilter for VibranceFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let vibrance_factor = self.vibrance / 100.0;
        
        for pixel in image.pixels_mut() {
            let rgb = Rgb::from_u8(pixel[0], pixel[1], pixel[2]);
            let hsl = rgb.to_hsl();
            
            // Vibrance works differently from saturation - it protects skin tones
            // and affects less-saturated colors more than already saturated ones
            
            // Calculate how saturated this pixel already is (0.0 to 1.0)
            let current_saturation = hsl.s;
            
            // Vibrance effect is stronger on less saturated pixels
            let vibrance_strength = if vibrance_factor > 0.0 {
                // Positive vibrance: affect less saturated colors more
                (1.0 - current_saturation) * vibrance_factor
            } else {
                // Negative vibrance: affect all colors but preserve some saturation
                vibrance_factor * (0.5 + current_saturation * 0.5)
            };
            
            // Apply vibrance adjustment
            let new_saturation = clamp_f32(
                current_saturation + vibrance_strength,
                0.0,
                1.0
            );
            
            let adjusted_hsl = Hsl::new(hsl.h, new_saturation, hsl.l);
            let adjusted_rgb = adjusted_hsl.to_rgb();
            
            let (r, g, b) = adjusted_rgb.to_u8();
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Vibrance"
    }
}

// Enhanced saturation filter that allows for more selective adjustment
#[wasm_bindgen]
pub struct SelectiveSaturationFilter {
    saturation: f32,        // -100.0 to 100.0
    hue_range_start: f32,   // 0.0 to 360.0
    hue_range_end: f32,     // 0.0 to 360.0
    feather: f32,           // 0.0 to 90.0 (transition smoothness)
}

#[wasm_bindgen]
impl SelectiveSaturationFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(saturation: f32, hue_range_start: f32, hue_range_end: f32, feather: Option<f32>) -> Self {
        Self {
            saturation: clamp_f32(saturation, -100.0, 100.0),
            hue_range_start: hue_range_start % 360.0,
            hue_range_end: hue_range_end % 360.0,
            feather: clamp_f32(feather.unwrap_or(30.0), 0.0, 90.0),
        }
    }

    // Preset methods for common color ranges
    pub fn reds(saturation: f32) -> Self {
        Self::new(saturation, 340.0, 20.0, Some(30.0))
    }

    pub fn oranges(saturation: f32) -> Self {
        Self::new(saturation, 10.0, 40.0, Some(20.0))
    }

    pub fn yellows(saturation: f32) -> Self {
        Self::new(saturation, 40.0, 70.0, Some(20.0))
    }

    pub fn greens(saturation: f32) -> Self {
        Self::new(saturation, 70.0, 150.0, Some(30.0))
    }

    pub fn blues(saturation: f32) -> Self {
        Self::new(saturation, 200.0, 260.0, Some(30.0))
    }

    pub fn purples(saturation: f32) -> Self {
        Self::new(saturation, 260.0, 320.0, Some(30.0))
    }

    fn hue_in_range(&self, hue: f32) -> f32 {
        let start = self.hue_range_start;
        let end = self.hue_range_end;
        
        let hue_distance = if start <= end {
            // Normal range (e.g., 60 to 120)
            if hue >= start && hue <= end {
                let center = (start + end) / 2.0;
                let range_half = (end - start) / 2.0;
                let distance_from_center = (hue - center).abs();
                1.0 - (distance_from_center / range_half).min(1.0)
            } else {
                0.0
            }
        } else {
            // Wrapped range (e.g., 340 to 20, crossing 0/360)
            if hue >= start || hue <= end {
                let adjusted_hue = if hue >= start { hue - 360.0 } else { hue };
                let adjusted_start = start - 360.0;
                let center = (adjusted_start + end) / 2.0;
                let range_half = (end - adjusted_start) / 2.0;
                let distance_from_center = (adjusted_hue - center).abs();
                1.0 - (distance_from_center / range_half).min(1.0)
            } else {
                0.0
            }
        };
        
        // Apply feathering
        let feather_factor = self.feather / 90.0;
        if hue_distance > feather_factor {
            1.0
        } else if hue_distance == 0.0 {
            0.0
        } else {
            hue_distance / feather_factor
        }
    }
}

impl ImageFilter for SelectiveSaturationFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let saturation_factor = 1.0 + (self.saturation / 100.0);
        
        for pixel in image.pixels_mut() {
            let rgb = Rgb::from_u8(pixel[0], pixel[1], pixel[2]);
            let hsl = rgb.to_hsl();
            
            // Check if this hue is in our target range
            let range_factor = self.hue_in_range(hsl.h);
            
            if range_factor > 0.0 {
                // Apply saturation adjustment proportional to how close we are to target range
                let adjusted_saturation = clamp_f32(
                    hsl.s + (hsl.s * (saturation_factor - 1.0) * range_factor),
                    0.0,
                    1.0
                );
                
                let adjusted_hsl = Hsl::new(hsl.h, adjusted_saturation, hsl.l);
                let adjusted_rgb = adjusted_hsl.to_rgb();
                
                let (r, g, b) = adjusted_rgb.to_u8();
                pixel[0] = r;
                pixel[1] = g;
                pixel[2] = b;
            }
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Selective Saturation"
    }
}
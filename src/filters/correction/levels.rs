use crate::filters::{ImageFilter, ImageData};
use crate::core::histogram::Histogram;
use crate::utils::clamp_f32;
use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;

pub struct LevelsFilter {
    input_shadow: f32,    // 0.0 to 1.0 (shadow input level)
    input_gamma: f32,     // 0.1 to 3.0 (gamma correction)
    input_highlight: f32, // 0.0 to 1.0 (highlight input level)
    output_shadow: f32,   // 0.0 to 1.0 (shadow output level)
    output_highlight: f32, // 0.0 to 1.0 (highlight output level)
}

impl LevelsFilter {
    pub fn new(
        input_shadow: f32,
        input_gamma: f32,
        input_highlight: f32,
        output_shadow: f32,
        output_highlight: f32,
    ) -> Self {
        Self {
            input_shadow: clamp_f32(input_shadow, 0.0, 1.0),
            input_gamma: clamp_f32(input_gamma, 0.1, 3.0),
            input_highlight: clamp_f32(input_highlight, 0.0, 1.0),
            output_shadow: clamp_f32(output_shadow, 0.0, 1.0),
            output_highlight: clamp_f32(output_highlight, 0.0, 1.0),
        }
    }

    pub fn input_shadow(&self) -> f32 {
        self.input_shadow
    }

    pub fn input_gamma(&self) -> f32 {
        self.input_gamma
    }

    pub fn input_highlight(&self) -> f32 {
        self.input_highlight
    }

    pub fn output_shadow(&self) -> f32 {
        self.output_shadow
    }

    pub fn output_highlight(&self) -> f32 {
        self.output_highlight
    }

    // Auto levels based on histogram (not exposed to WASM)
    pub fn auto_levels_from_image(image: &ImageData) -> Self {
        let histogram = Histogram::from_image(image);
        let (shadow, gamma, highlight) = histogram.calculate_auto_levels();
        
        Self::new(shadow, gamma, highlight, 0.0, 1.0)
    }

    // Reset to default (no adjustment)
    pub fn reset() -> Self {
        Self::new(0.0, 1.0, 1.0, 0.0, 1.0)
    }

    fn apply_levels_curve(&self, value: f32) -> f32 {
        let input_range = self.input_highlight - self.input_shadow;
        if input_range <= 0.0 {
            return value;
        }
        
        // Input level adjustment
        let normalized = ((value - self.input_shadow) / input_range).clamp(0.0, 1.0);
        
        // Gamma correction
        let gamma_corrected = if self.input_gamma != 1.0 {
            normalized.powf(1.0 / self.input_gamma)
        } else {
            normalized
        };
        
        // Output level adjustment
        let output_range = self.output_highlight - self.output_shadow;
        self.output_shadow + gamma_corrected * output_range
    }
}

impl ImageFilter for LevelsFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        for pixel in image.pixels_mut() {
            for c in 0..3 { // Process RGB channels, skip alpha
                let original_value = pixel[c] as f32 / 255.0;
                let adjusted_value = self.apply_levels_curve(original_value);
                pixel[c] = (adjusted_value.clamp(0.0, 1.0) * 255.0) as u8;
            }
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Levels"
    }
}

#[wasm_bindgen]
pub struct CurvesFilter {
    // Simplified curves filter with control points
    shadows: f32,    // -1.0 to 1.0 (adjustment for shadows)
    midtones: f32,   // -1.0 to 1.0 (adjustment for midtones)
    highlights: f32, // -1.0 to 1.0 (adjustment for highlights)
}

#[wasm_bindgen]
impl CurvesFilter {
    pub fn new(shadows: f32, midtones: f32, highlights: f32) -> Self {
        Self {
            shadows: clamp_f32(shadows, -1.0, 1.0),
            midtones: clamp_f32(midtones, -1.0, 1.0),
            highlights: clamp_f32(highlights, -1.0, 1.0),
        }
    }

    pub fn shadows(&self) -> f32 {
        self.shadows
    }

    pub fn midtones(&self) -> f32 {
        self.midtones
    }

    pub fn highlights(&self) -> f32 {
        self.highlights
    }

    pub fn reset() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn apply_curve(&self, value: f32) -> f32 {
        let x = value.clamp(0.0, 1.0);
        
        // Define control points
        let p0 = (0.0, 0.0 + self.shadows * 0.2); // Shadow point
        let p1 = (0.5, 0.5 + self.midtones * 0.3); // Midtone point
        let p2 = (1.0, 1.0 + self.highlights * 0.2); // Highlight point
        
        // Quadratic Bézier curve interpolation
        let t = if x <= 0.5 {
            x * 2.0 // 0 to 1 for first segment
        } else {
            (x - 0.5) * 2.0 // 0 to 1 for second segment
        };
        
        let result = if x <= 0.5 {
            // Interpolate between p0 and p1
            let y = (1.0 - t).powi(2) * p0.1 + 
                    2.0 * (1.0 - t) * t * (p0.1 + p1.1) * 0.5 + 
                    t.powi(2) * p1.1;
            y
        } else {
            // Interpolate between p1 and p2
            let y = (1.0 - t).powi(2) * p1.1 + 
                    2.0 * (1.0 - t) * t * (p1.1 + p2.1) * 0.5 + 
                    t.powi(2) * p2.1;
            y
        };
        
        result.clamp(0.0, 1.0)
    }
}

impl ImageFilter for CurvesFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        // Skip if no adjustments
        if self.shadows.abs() < 0.01 && self.midtones.abs() < 0.01 && self.highlights.abs() < 0.01 {
            return Ok(());
        }
        
        for pixel in image.pixels_mut() {
            for c in 0..3 { // Process RGB channels, skip alpha
                let original_value = pixel[c] as f32 / 255.0;
                let adjusted_value = self.apply_curve(original_value);
                pixel[c] = (adjusted_value * 255.0) as u8;
            }
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Curves"
    }
}
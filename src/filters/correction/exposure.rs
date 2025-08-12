use crate::filters::{ImageFilter, ImageData};
use crate::utils::clamp_f32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExposureFilter {
    exposure: f32,    // -5.0 to 5.0 (stops of exposure)
    highlights: f32,  // -100.0 to 0.0 (highlight recovery)
    shadows: f32,     // 0.0 to 100.0 (shadow lift)
}

#[wasm_bindgen]
impl ExposureFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(exposure: f32, highlights: f32, shadows: f32) -> Self {
        Self {
            exposure: clamp_f32(exposure, -5.0, 5.0),
            highlights: clamp_f32(highlights, -100.0, 0.0),
            shadows: clamp_f32(shadows, 0.0, 100.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn exposure(&self) -> f32 {
        self.exposure
    }

    #[wasm_bindgen(getter)]
    pub fn highlights(&self) -> f32 {
        self.highlights
    }

    #[wasm_bindgen(getter)]
    pub fn shadows(&self) -> f32 {
        self.shadows
    }

    #[wasm_bindgen(setter)]
    pub fn set_exposure(&mut self, exposure: f32) {
        self.exposure = clamp_f32(exposure, -5.0, 5.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_highlights(&mut self, highlights: f32) {
        self.highlights = clamp_f32(highlights, -100.0, 0.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_shadows(&mut self, shadows: f32) {
        self.shadows = clamp_f32(shadows, 0.0, 100.0);
    }

    pub fn reset() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl ImageFilter for ExposureFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        // Skip if no adjustments
        if self.exposure.abs() < 0.01 && self.highlights.abs() < 1.0 && self.shadows < 1.0 {
            return Ok(());
        }

        // Convert exposure stops to multiplier
        let exposure_multiplier = 2.0f32.powf(self.exposure);
        let highlight_factor = 1.0 + self.highlights / 100.0;
        let shadow_factor = self.shadows / 100.0;

        for pixel in image.pixels_mut() {
            for c in 0..3 { // Process RGB channels, skip alpha
                let original_value = pixel[c] as f32 / 255.0;
                
                // Apply exposure
                let exposed = original_value * exposure_multiplier;
                
                // Apply highlight recovery (reduces bright areas)
                let highlight_mask = original_value.powf(2.0); // More effect on bright pixels
                let highlight_adjusted = exposed * (1.0 - highlight_mask * (1.0 - highlight_factor));
                
                // Apply shadow lift (brightens dark areas)
                let shadow_mask = (1.0 - original_value).powf(2.0); // More effect on dark pixels
                let shadow_lift = shadow_mask * shadow_factor * 0.3;
                let final_value = (highlight_adjusted + shadow_lift).clamp(0.0, 1.0);
                
                pixel[c] = (final_value * 255.0) as u8;
            }
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Exposure"
    }
}

#[wasm_bindgen]
pub struct ShadowHighlightFilter {
    shadow_amount: f32,      // 0.0 to 100.0 (shadow brightening)
    highlight_amount: f32,   // 0.0 to 100.0 (highlight darkening)
    shadow_width: f32,       // 10.0 to 100.0 (tonal width for shadows)
    highlight_width: f32,    // 10.0 to 100.0 (tonal width for highlights)
    radius: f32,             // 0.5 to 10.0 (mask radius)
}

#[wasm_bindgen]
impl ShadowHighlightFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        shadow_amount: f32,
        highlight_amount: f32,
        shadow_width: Option<f32>,
        highlight_width: Option<f32>,
        radius: Option<f32>,
    ) -> Self {
        Self {
            shadow_amount: clamp_f32(shadow_amount, 0.0, 100.0),
            highlight_amount: clamp_f32(highlight_amount, 0.0, 100.0),
            shadow_width: clamp_f32(shadow_width.unwrap_or(50.0), 10.0, 100.0),
            highlight_width: clamp_f32(highlight_width.unwrap_or(50.0), 10.0, 100.0),
            radius: clamp_f32(radius.unwrap_or(2.0), 0.5, 10.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn shadow_amount(&self) -> f32 {
        self.shadow_amount
    }

    #[wasm_bindgen(getter)]
    pub fn highlight_amount(&self) -> f32 {
        self.highlight_amount
    }

    #[wasm_bindgen(getter)]
    pub fn shadow_width(&self) -> f32 {
        self.shadow_width
    }

    #[wasm_bindgen(getter)]
    pub fn highlight_width(&self) -> f32 {
        self.highlight_width
    }

    #[wasm_bindgen(getter)]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    // Preset adjustments
    pub fn recover_shadows(amount: f32) -> Self {
        Self::new(amount, 0.0, None, None, None)
    }

    pub fn recover_highlights(amount: f32) -> Self {
        Self::new(0.0, amount, None, None, None)
    }

    pub fn balanced_recovery(shadow_amount: f32, highlight_amount: f32) -> Self {
        Self::new(shadow_amount, highlight_amount, None, None, None)
    }
}

impl ImageFilter for ShadowHighlightFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        // Skip if no adjustments
        if self.shadow_amount < 1.0 && self.highlight_amount < 1.0 {
            return Ok(());
        }

        let width = image.width();
        let height = image.height();
        
        // Create luminance mask for better edge detection
        let mut luminance_data = Vec::with_capacity((width * height) as usize);
        
        for pixel in image.pixels() {
            // Calculate luminance using ITU-R BT.709 weights
            let luminance = (0.2126 * pixel[0] as f32 + 
                           0.7152 * pixel[1] as f32 + 
                           0.0722 * pixel[2] as f32) / 255.0;
            luminance_data.push(luminance);
        }
        
        // Apply shadow/highlight adjustments
        for (i, pixel) in image.pixels_mut().enumerate() {
            let luminance = luminance_data[i];
            
            // Calculate shadow mask (affects dark areas more)
            let shadow_threshold = self.shadow_width / 100.0;
            let shadow_mask = if luminance < shadow_threshold {
                1.0 - (luminance / shadow_threshold)
            } else {
                0.0
            };
            
            // Calculate highlight mask (affects bright areas more)
            let highlight_threshold = 1.0 - (self.highlight_width / 100.0);
            let highlight_mask = if luminance > highlight_threshold {
                (luminance - highlight_threshold) / (1.0 - highlight_threshold)
            } else {
                0.0
            };
            
            for c in 0..3 { // Process RGB channels, skip alpha
                let original_value = pixel[c] as f32 / 255.0;
                let mut adjusted_value = original_value;
                
                // Apply shadow adjustment
                if shadow_mask > 0.0 && self.shadow_amount > 0.0 {
                    let shadow_lift = shadow_mask * (self.shadow_amount / 100.0) * 0.3;
                    adjusted_value = (adjusted_value + shadow_lift).min(1.0);
                }
                
                // Apply highlight adjustment
                if highlight_mask > 0.0 && self.highlight_amount > 0.0 {
                    let highlight_reduction = highlight_mask * (self.highlight_amount / 100.0) * 0.2;
                    adjusted_value = (adjusted_value * (1.0 - highlight_reduction)).max(0.0);
                }
                
                pixel[c] = (adjusted_value * 255.0) as u8;
            }
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Shadow/Highlight"
    }
}
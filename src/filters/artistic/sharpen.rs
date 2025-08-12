use crate::filters::{ImageFilter, ImageData};
use crate::utils::clamp_f32;
use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SharpenFilter {
    strength: f32, // 0.0 to 10.0
}

#[wasm_bindgen]
impl SharpenFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(strength: f32) -> Self {
        Self {
            strength: clamp_f32(strength, 0.0, 10.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn strength(&self) -> f32 {
        self.strength
    }

    #[wasm_bindgen(setter)]
    pub fn set_strength(&mut self, strength: f32) {
        self.strength = clamp_f32(strength, 0.0, 10.0);
    }

    // Preset sharpening levels
    pub fn light() -> Self {
        Self::new(0.5)
    }

    pub fn medium() -> Self {
        Self::new(1.0)
    }

    pub fn strong() -> Self {
        Self::new(2.0)
    }
}

impl ImageFilter for SharpenFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        if self.strength <= 0.01 {
            return Ok(()); // No sharpening needed
        }

        let width = image.width();
        let height = image.height();
        let original_data = image.as_raw().clone();
        
        // Basic unsharp mask kernel (3x3)
        let kernel = [
            [0.0, -self.strength, 0.0],
            [-self.strength, 1.0 + 4.0 * self.strength, -self.strength],
            [0.0, -self.strength, 0.0],
        ];
        
        let mut sharpened_data = vec![0u8; original_data.len()];
        
        for y in 0..height {
            for x in 0..width {
                let mut sum = [0.0f32; 4];
                
                // Apply kernel
                for ky in 0..3 {
                    for kx in 0..3 {
                        let sample_x = (x as i32 + kx as i32 - 1).max(0).min(width as i32 - 1) as u32;
                        let sample_y = (y as i32 + ky as i32 - 1).max(0).min(height as i32 - 1) as u32;
                        
                        let sample_idx = ((sample_y * width + sample_x) * 4) as usize;
                        let weight = kernel[ky][kx];
                        
                        for c in 0..3 { // Don't modify alpha
                            sum[c] += original_data[sample_idx + c] as f32 * weight;
                        }
                    }
                }
                
                let dst_idx = ((y * width + x) * 4) as usize;
                for c in 0..3 {
                    sharpened_data[dst_idx + c] = sum[c].round().clamp(0.0, 255.0) as u8;
                }
                sharpened_data[dst_idx + 3] = original_data[dst_idx + 3]; // Keep alpha unchanged
            }
        }
        
        *image = ImageBuffer::from_raw(width, height, sharpened_data)
            .ok_or_else(|| JsValue::from_str("Failed to create sharpened image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Sharpen"
    }
}

#[wasm_bindgen]
pub struct UnsharpMaskFilter {
    amount: f32,      // 0.0 to 5.0 - sharpening strength
    radius: f32,      // 0.1 to 10.0 - blur radius for mask
    threshold: f32,   // 0.0 to 255.0 - threshold for applying sharpening
}

#[wasm_bindgen]
impl UnsharpMaskFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(amount: f32, radius: f32, threshold: f32) -> Self {
        Self {
            amount: clamp_f32(amount, 0.0, 5.0),
            radius: clamp_f32(radius, 0.1, 10.0),
            threshold: clamp_f32(threshold, 0.0, 255.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> f32 {
        self.amount
    }

    #[wasm_bindgen(getter)]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    #[wasm_bindgen(getter)]
    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    #[wasm_bindgen(setter)]
    pub fn set_amount(&mut self, amount: f32) {
        self.amount = clamp_f32(amount, 0.0, 5.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = clamp_f32(radius, 0.1, 10.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = clamp_f32(threshold, 0.0, 255.0);
    }

    // Preset unsharp mask settings
    pub fn web() -> Self {
        Self::new(0.5, 1.0, 2.0)
    }

    pub fn print() -> Self {
        Self::new(1.0, 2.0, 3.0)
    }

    pub fn portrait() -> Self {
        Self::new(0.75, 2.0, 3.0)
    }
}

impl ImageFilter for UnsharpMaskFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        if self.amount <= 0.01 {
            return Ok(()); // No sharpening needed
        }

        let width = image.width();
        let height = image.height();
        let original_data = image.as_raw().clone();
        
        // Create blurred version for the mask
        let mut blurred_image = image.clone();
        let blur_filter = crate::filters::artistic::GaussianBlurFilter::new(self.radius);
        blur_filter.apply(&mut blurred_image)?;
        let blurred_data = blurred_image.as_raw();
        
        let mut sharpened_data = vec![0u8; original_data.len()];
        
        for i in (0..original_data.len()).step_by(4) {
            for c in 0..3 { // Process RGB channels, skip alpha
                let original = original_data[i + c] as f32;
                let blurred = blurred_data[i + c] as f32;
                
                // Calculate the difference (edge detection)
                let difference = original - blurred;
                
                // Apply threshold
                if difference.abs() >= self.threshold {
                    // Apply unsharp mask formula
                    let sharpened = original + self.amount * difference;
                    sharpened_data[i + c] = sharpened.round().clamp(0.0, 255.0) as u8;
                } else {
                    sharpened_data[i + c] = original_data[i + c];
                }
            }
            sharpened_data[i + 3] = original_data[i + 3]; // Keep alpha unchanged
        }
        
        *image = ImageBuffer::from_raw(width, height, sharpened_data)
            .ok_or_else(|| JsValue::from_str("Failed to create unsharp masked image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Unsharp Mask"
    }
}

#[wasm_bindgen]
pub struct EdgeEnhanceFilter {
    strength: f32,    // 0.0 to 5.0
    threshold: f32,   // 0.0 to 255.0
}

#[wasm_bindgen]
impl EdgeEnhanceFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(strength: f32, threshold: f32) -> Self {
        Self {
            strength: clamp_f32(strength, 0.0, 5.0),
            threshold: clamp_f32(threshold, 0.0, 255.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn strength(&self) -> f32 {
        self.strength
    }

    #[wasm_bindgen(getter)]
    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    pub fn light() -> Self {
        Self::new(0.5, 10.0)
    }

    pub fn medium() -> Self {
        Self::new(1.0, 15.0)
    }

    pub fn strong() -> Self {
        Self::new(2.0, 20.0)
    }
}

impl ImageFilter for EdgeEnhanceFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        if self.strength <= 0.01 {
            return Ok(()); // No enhancement needed
        }

        let width = image.width();
        let height = image.height();
        let original_data = image.as_raw().clone();
        
        // Sobel edge detection kernels
        let sobel_x = [
            [-1.0, 0.0, 1.0],
            [-2.0, 0.0, 2.0],
            [-1.0, 0.0, 1.0],
        ];
        
        let sobel_y = [
            [-1.0, -2.0, -1.0],
            [0.0,  0.0,  0.0],
            [1.0,  2.0,  1.0],
        ];
        
        let mut enhanced_data = original_data.clone();
        
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut edge_strength = [0.0f32; 3];
                
                // Apply Sobel operators
                for c in 0..3 { // Process RGB channels
                    let mut grad_x = 0.0f32;
                    let mut grad_y = 0.0f32;
                    
                    for ky in 0..3 {
                        for kx in 0..3 {
                            let sample_x = x + kx - 1;
                            let sample_y = y + ky - 1;
                            let sample_idx = ((sample_y * width + sample_x) * 4 + c) as usize;
                            let pixel_value = original_data[sample_idx] as f32;
                            
                            grad_x += pixel_value * sobel_x[ky as usize][kx as usize];
                            grad_y += pixel_value * sobel_y[ky as usize][kx as usize];
                        }
                    }
                    
                    edge_strength[c as usize] = (grad_x * grad_x + grad_y * grad_y).sqrt();
                }
                
                // Calculate overall edge strength (luminance-weighted)
                let overall_strength = 0.299 * edge_strength[0] + 
                                     0.587 * edge_strength[1] + 
                                     0.114 * edge_strength[2];
                
                let dst_idx = ((y * width + x) * 4) as usize;
                
                if overall_strength >= self.threshold {
                    // Enhance edges
                    for c in 0..3 {
                        let original = original_data[dst_idx + c] as f32;
                        let enhancement = edge_strength[c] * self.strength / 255.0;
                        let enhanced = original + enhancement * (255.0 - original);
                        enhanced_data[dst_idx + c] = enhanced.round().clamp(0.0, 255.0) as u8;
                    }
                }
                // Alpha remains unchanged
            }
        }
        
        *image = ImageBuffer::from_raw(width, height, enhanced_data)
            .ok_or_else(|| JsValue::from_str("Failed to create edge enhanced image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Edge Enhance"
    }
}
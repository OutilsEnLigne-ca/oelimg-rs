use crate::filters::{ImageFilter, ImageData};
use crate::utils::clamp_f32;
use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GaussianBlurFilter {
    radius: f32,
}

#[wasm_bindgen]
impl GaussianBlurFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(radius: f32) -> Self {
        Self {
            radius: clamp_f32(radius, 0.0, 100.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    #[wasm_bindgen(setter)]
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = clamp_f32(radius, 0.0, 100.0);
    }

    // Preset blur effects
    pub fn light() -> Self {
        Self::new(1.0)
    }

    pub fn medium() -> Self {
        Self::new(3.0)
    }

    pub fn heavy() -> Self {
        Self::new(8.0)
    }
}

impl ImageFilter for GaussianBlurFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        if self.radius <= 0.1 {
            return Ok(()); // No blur needed
        }

        let width = image.width();
        let height = image.height();
        
        // Create Gaussian kernel
        let kernel_size = (self.radius * 2.0).ceil() as usize * 2 + 1;
        let kernel = create_gaussian_kernel(self.radius, kernel_size);
        let kernel_half = kernel_size / 2;
        
        // Apply horizontal blur
        let mut temp_data = vec![0u8; (width * height * 4) as usize];
        
        for y in 0..height {
            for x in 0..width {
                let mut sum = [0.0f32; 4];
                let mut weight_sum = 0.0f32;
                
                for i in 0..kernel_size {
                    let offset = i as i32 - kernel_half as i32;
                    let sample_x = (x as i32 + offset).max(0).min(width as i32 - 1) as u32;
                    
                    let pixel = image.get_pixel(sample_x, y);
                    let weight = kernel[i];
                    
                    for c in 0..4 {
                        sum[c] += pixel[c] as f32 * weight;
                    }
                    weight_sum += weight;
                }
                
                let dst_idx = ((y * width + x) * 4) as usize;
                for c in 0..4 {
                    temp_data[dst_idx + c] = (sum[c] / weight_sum).round().clamp(0.0, 255.0) as u8;
                }
            }
        }
        
        // Apply vertical blur
        let temp_image: ImageData = ImageBuffer::from_raw(width, height, temp_data)
            .ok_or_else(|| JsValue::from_str("Failed to create temporary image buffer"))?;
            
        let mut final_data = vec![0u8; (width * height * 4) as usize];
        
        for y in 0..height {
            for x in 0..width {
                let mut sum = [0.0f32; 4];
                let mut weight_sum = 0.0f32;
                
                for i in 0..kernel_size {
                    let offset = i as i32 - kernel_half as i32;
                    let sample_y = (y as i32 + offset).max(0).min(height as i32 - 1) as u32;
                    
                    let pixel = temp_image.get_pixel(x, sample_y).0;
                    let weight = kernel[i];
                    
                    for c in 0..4 {
                        sum[c] += pixel[c] as f32 * weight;
                    }
                    weight_sum += weight;
                }
                
                let dst_idx = ((y * width + x) * 4) as usize;
                for c in 0..4 {
                    final_data[dst_idx + c] = (sum[c] / weight_sum).round().clamp(0.0, 255.0) as u8;
                }
            }
        }
        
        *image = ImageBuffer::from_raw(width, height, final_data)
            .ok_or_else(|| JsValue::from_str("Failed to create blurred image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Gaussian Blur"
    }
}

#[wasm_bindgen]
pub struct MotionBlurFilter {
    distance: f32,
    angle_degrees: f32,
}

#[wasm_bindgen]
impl MotionBlurFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(distance: f32, angle_degrees: f32) -> Self {
        Self {
            distance: clamp_f32(distance, 0.0, 100.0),
            angle_degrees: angle_degrees % 360.0,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn distance(&self) -> f32 {
        self.distance
    }

    #[wasm_bindgen(getter)]
    pub fn angle_degrees(&self) -> f32 {
        self.angle_degrees
    }

    #[wasm_bindgen(setter)]
    pub fn set_distance(&mut self, distance: f32) {
        self.distance = clamp_f32(distance, 0.0, 100.0);
    }

    #[wasm_bindgen(setter)]
    pub fn set_angle_degrees(&mut self, angle: f32) {
        self.angle_degrees = angle % 360.0;
    }

    // Preset motion blur effects
    pub fn horizontal(distance: f32) -> Self {
        Self::new(distance, 0.0)
    }

    pub fn vertical(distance: f32) -> Self {
        Self::new(distance, 90.0)
    }
}

impl ImageFilter for MotionBlurFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        if self.distance <= 0.1 {
            return Ok(()); // No blur needed
        }

        let width = image.width();
        let height = image.height();
        let original_data = image.as_raw().clone();
        
        // Calculate motion vector
        let angle_rad = self.angle_degrees * std::f32::consts::PI / 180.0;
        let dx = angle_rad.cos() * self.distance;
        let dy = angle_rad.sin() * self.distance;
        
        // Number of samples along the motion path
        let samples = (self.distance * 2.0).ceil() as i32 + 1;
        let step_x = dx / samples as f32;
        let step_y = dy / samples as f32;
        
        let mut blurred_data = vec![0u8; original_data.len()];
        
        for y in 0..height {
            for x in 0..width {
                let mut sum = [0.0f32; 4];
                let mut count = 0;
                
                // Sample along the motion path
                for i in 0..samples {
                    let offset_x = step_x * (i as f32 - samples as f32 / 2.0);
                    let offset_y = step_y * (i as f32 - samples as f32 / 2.0);
                    
                    let sample_x = (x as f32 + offset_x).round() as i32;
                    let sample_y = (y as f32 + offset_y).round() as i32;
                    
                    if sample_x >= 0 && sample_x < width as i32 && 
                       sample_y >= 0 && sample_y < height as i32 {
                        let sample_idx = ((sample_y as u32 * width + sample_x as u32) * 4) as usize;
                        
                        for c in 0..4 {
                            sum[c] += original_data[sample_idx + c] as f32;
                        }
                        count += 1;
                    }
                }
                
                let dst_idx = ((y * width + x) * 4) as usize;
                if count > 0 {
                    for c in 0..4 {
                        blurred_data[dst_idx + c] = (sum[c] / count as f32).round().clamp(0.0, 255.0) as u8;
                    }
                } else {
                    // Copy original pixel if no valid samples
                    for c in 0..4 {
                        blurred_data[dst_idx + c] = original_data[dst_idx + c];
                    }
                }
            }
        }
        
        *image = ImageBuffer::from_raw(width, height, blurred_data)
            .ok_or_else(|| JsValue::from_str("Failed to create motion blurred image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Motion Blur"
    }
}

#[wasm_bindgen]
pub struct RadialBlurFilter {
    center_x: f32, // 0.0 to 1.0 (percentage of image width)
    center_y: f32, // 0.0 to 1.0 (percentage of image height)
    strength: f32, // 0.0 to 100.0
}

#[wasm_bindgen]
impl RadialBlurFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(center_x: f32, center_y: f32, strength: f32) -> Self {
        Self {
            center_x: clamp_f32(center_x, 0.0, 1.0),
            center_y: clamp_f32(center_y, 0.0, 1.0),
            strength: clamp_f32(strength, 0.0, 100.0),
        }
    }

    // Center radial blur
    pub fn center(strength: f32) -> Self {
        Self::new(0.5, 0.5, strength)
    }

    #[wasm_bindgen(getter)]
    pub fn center_x(&self) -> f32 {
        self.center_x
    }

    #[wasm_bindgen(getter)]
    pub fn center_y(&self) -> f32 {
        self.center_y
    }

    #[wasm_bindgen(getter)]
    pub fn strength(&self) -> f32 {
        self.strength
    }
}

impl ImageFilter for RadialBlurFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        if self.strength <= 0.1 {
            return Ok(()); // No blur needed
        }

        let width = image.width();
        let height = image.height();
        let original_data = image.as_raw().clone();
        
        let center_x = self.center_x * width as f32;
        let center_y = self.center_y * height as f32;
        let max_distance = ((width * width + height * height) as f32).sqrt() / 2.0;
        
        let mut blurred_data = vec![0u8; original_data.len()];
        
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // Calculate blur amount based on distance from center
                let blur_amount = (distance / max_distance) * self.strength;
                let samples = (blur_amount * 0.5).ceil() as i32 + 1;
                
                if samples <= 1 {
                    // No blur, copy original pixel
                    let src_idx = ((y * width + x) * 4) as usize;
                    let dst_idx = src_idx;
                    for c in 0..4 {
                        blurred_data[dst_idx + c] = original_data[src_idx + c];
                    }
                    continue;
                }
                
                let mut sum = [0.0f32; 4];
                let mut count = 0;
                
                // Sample in a small radius around the pixel
                let sample_radius = blur_amount * 0.5;
                
                for i in 0..samples {
                    for j in 0..samples {
                        let offset_x = ((i as f32 - samples as f32 / 2.0) / samples as f32) * sample_radius;
                        let offset_y = ((j as f32 - samples as f32 / 2.0) / samples as f32) * sample_radius;
                        
                        let sample_x = (x as f32 + offset_x).round() as i32;
                        let sample_y = (y as f32 + offset_y).round() as i32;
                        
                        if sample_x >= 0 && sample_x < width as i32 && 
                           sample_y >= 0 && sample_y < height as i32 {
                            let sample_idx = ((sample_y as u32 * width + sample_x as u32) * 4) as usize;
                            
                            for c in 0..4 {
                                sum[c] += original_data[sample_idx + c] as f32;
                            }
                            count += 1;
                        }
                    }
                }
                
                let dst_idx = ((y * width + x) * 4) as usize;
                if count > 0 {
                    for c in 0..4 {
                        blurred_data[dst_idx + c] = (sum[c] / count as f32).round().clamp(0.0, 255.0) as u8;
                    }
                } else {
                    for c in 0..4 {
                        blurred_data[dst_idx + c] = original_data[dst_idx + c];
                    }
                }
            }
        }
        
        *image = ImageBuffer::from_raw(width, height, blurred_data)
            .ok_or_else(|| JsValue::from_str("Failed to create radial blurred image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Radial Blur"
    }
}

// Helper function to create Gaussian kernel
fn create_gaussian_kernel(radius: f32, size: usize) -> Vec<f32> {
    let mut kernel = vec![0.0; size];
    let sigma = radius / 3.0; // Standard deviation
    let two_sigma_squared = 2.0 * sigma * sigma;
    let half_size = size / 2;
    
    let mut sum = 0.0;
    
    for i in 0..size {
        let x = (i as f32 - half_size as f32).abs();
        let value = (-x * x / two_sigma_squared).exp();
        kernel[i] = value;
        sum += value;
    }
    
    // Normalize kernel
    for i in 0..size {
        kernel[i] /= sum;
    }
    
    kernel
}
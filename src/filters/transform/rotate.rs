use crate::filters::{ImageFilter, ImageData};
use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

#[wasm_bindgen]
pub struct RotateFilter {
    angle_degrees: f32,
    background_color: [u8; 4], // RGBA background color for areas outside the rotated image
}

#[wasm_bindgen]
impl RotateFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(angle_degrees: f32, background_color: Option<Vec<u8>>) -> Result<RotateFilter, JsValue> {
        let bg_color = if let Some(color) = background_color {
            if color.len() != 4 {
                return Err(JsValue::from_str("Background color must have 4 components (RGBA)"));
            }
            [color[0], color[1], color[2], color[3]]
        } else {
            [0, 0, 0, 0] // Transparent black by default
        };

        Ok(RotateFilter {
            angle_degrees: angle_degrees % 360.0,
            background_color: bg_color,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn angle_degrees(&self) -> f32 {
        self.angle_degrees
    }

    #[wasm_bindgen(getter)]
    pub fn background_color(&self) -> Vec<u8> {
        self.background_color.to_vec()
    }

    #[wasm_bindgen(setter)]
    pub fn set_angle_degrees(&mut self, angle: f32) {
        self.angle_degrees = angle % 360.0;
    }

    #[wasm_bindgen(setter)]
    pub fn set_background_color(&mut self, color: Vec<u8>) -> Result<(), JsValue> {
        if color.len() != 4 {
            return Err(JsValue::from_str("Background color must have 4 components (RGBA)"));
        }
        self.background_color = [color[0], color[1], color[2], color[3]];
        Ok(())
    }

    // Rotate 90 degrees clockwise
    pub fn rotate_90_cw() -> Self {
        Self::new(90.0, None).unwrap()
    }

    // Rotate 90 degrees counter-clockwise
    pub fn rotate_90_ccw() -> Self {
        Self::new(-90.0, None).unwrap()
    }

    // Rotate 180 degrees
    pub fn rotate_180() -> Self {
        Self::new(180.0, None).unwrap()
    }

    fn calculate_rotated_dimensions(&self, width: u32, height: u32) -> (u32, u32) {
        let angle_rad = self.angle_degrees * PI / 180.0;
        let cos_angle = angle_rad.cos().abs();
        let sin_angle = angle_rad.sin().abs();
        
        let new_width = (width as f32 * cos_angle + height as f32 * sin_angle).ceil() as u32;
        let new_height = (height as f32 * cos_angle + width as f32 * sin_angle).ceil() as u32;
        
        (new_width, new_height)
    }
}

impl ImageFilter for RotateFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let original_width = image.width();
        let original_height = image.height();
        
        // Handle special cases for efficiency
        match self.angle_degrees as i32 {
            0 => return Ok(()), // No rotation needed
            90 | -270 => {
                rotate_90_cw(image);
                return Ok(());
            },
            180 | -180 => {
                rotate_180(image);
                return Ok(());
            },
            270 | -90 => {
                rotate_90_ccw(image);
                return Ok(());
            },
            _ => {} // Continue with arbitrary angle rotation
        }
        
        let angle_rad = self.angle_degrees * PI / 180.0;
        let cos_angle = angle_rad.cos();
        let sin_angle = angle_rad.sin();
        
        // Calculate new dimensions
        let (new_width, new_height) = self.calculate_rotated_dimensions(original_width, original_height);
        
        // Center points
        let cx = original_width as f32 / 2.0;
        let cy = original_height as f32 / 2.0;
        let new_cx = new_width as f32 / 2.0;
        let new_cy = new_height as f32 / 2.0;
        
        // Create new image buffer
        let mut rotated_data = vec![0u8; (new_width * new_height * 4) as usize];
        
        // Perform rotation using reverse mapping (from destination to source)
        for dst_y in 0..new_height {
            for dst_x in 0..new_width {
                // Translate to center, rotate, then translate back
                let dx = dst_x as f32 - new_cx;
                let dy = dst_y as f32 - new_cy;
                
                // Reverse rotation to find source coordinates
                let src_x = dx * cos_angle + dy * sin_angle + cx;
                let src_y = -dx * sin_angle + dy * cos_angle + cy;
                
                let dst_idx = ((dst_y * new_width + dst_x) * 4) as usize;
                
                // Bilinear interpolation for smooth results
                if src_x >= 0.0 && src_x < original_width as f32 && src_y >= 0.0 && src_y < original_height as f32 {
                    let color = bilinear_interpolate(image, src_x, src_y);
                    rotated_data[dst_idx..dst_idx + 4].copy_from_slice(&color);
                } else {
                    // Outside source image bounds - use background color
                    rotated_data[dst_idx..dst_idx + 4].copy_from_slice(&self.background_color);
                }
            }
        }
        
        // Replace the image data
        *image = ImageBuffer::from_raw(new_width, new_height, rotated_data)
            .ok_or_else(|| JsValue::from_str("Failed to create rotated image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Rotate"
    }
}

// Flip filter for horizontal and vertical mirroring
#[wasm_bindgen]
pub struct FlipFilter {
    horizontal: bool,
    vertical: bool,
}

#[wasm_bindgen]
impl FlipFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(horizontal: bool, vertical: bool) -> Self {
        FlipFilter {
            horizontal,
            vertical,
        }
    }

    pub fn horizontal() -> Self {
        Self::new(true, false)
    }

    pub fn vertical() -> Self {
        Self::new(false, true)
    }

    pub fn both() -> Self {
        Self::new(true, true)
    }

    #[wasm_bindgen(getter)]
    pub fn is_horizontal(&self) -> bool {
        self.horizontal
    }

    #[wasm_bindgen(getter)]
    pub fn is_vertical(&self) -> bool {
        self.vertical
    }
}

impl ImageFilter for FlipFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let width = image.width();
        let height = image.height();
        let mut flipped_data = image.as_raw().clone();
        
        if self.horizontal {
            for y in 0..height {
                for x in 0..width / 2 {
                    let left_idx = ((y * width + x) * 4) as usize;
                    let right_idx = ((y * width + (width - 1 - x)) * 4) as usize;
                    
                    // Swap pixels
                    for i in 0..4 {
                        flipped_data.swap(left_idx + i, right_idx + i);
                    }
                }
            }
        }
        
        if self.vertical {
            for y in 0..height / 2 {
                for x in 0..width {
                    let top_idx = ((y * width + x) * 4) as usize;
                    let bottom_idx = (((height - 1 - y) * width + x) * 4) as usize;
                    
                    // Swap pixels
                    for i in 0..4 {
                        flipped_data.swap(top_idx + i, bottom_idx + i);
                    }
                }
            }
        }
        
        *image = ImageBuffer::from_raw(width, height, flipped_data)
            .ok_or_else(|| JsValue::from_str("Failed to create flipped image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        if self.horizontal && self.vertical {
            "Flip Both"
        } else if self.horizontal {
            "Flip Horizontal"
        } else if self.vertical {
            "Flip Vertical"
        } else {
            "No Flip"
        }
    }
}

// Helper functions for efficient 90-degree rotations
fn rotate_90_cw(image: &mut ImageData) {
    let original_width = image.width();
    let original_height = image.height();
    let original_data = image.as_raw().clone();
    
    let mut rotated_data = vec![0u8; original_data.len()];
    
    for y in 0..original_height {
        for x in 0..original_width {
            let src_idx = ((y * original_width + x) * 4) as usize;
            let dst_x = original_height - 1 - y;
            let dst_y = x;
            let dst_idx = ((dst_y * original_height + dst_x) * 4) as usize;
            
            rotated_data[dst_idx..dst_idx + 4].copy_from_slice(&original_data[src_idx..src_idx + 4]);
        }
    }
    
    *image = ImageBuffer::from_raw(original_height, original_width, rotated_data).unwrap();
}

fn rotate_90_ccw(image: &mut ImageData) {
    let original_width = image.width();
    let original_height = image.height();
    let original_data = image.as_raw().clone();
    
    let mut rotated_data = vec![0u8; original_data.len()];
    
    for y in 0..original_height {
        for x in 0..original_width {
            let src_idx = ((y * original_width + x) * 4) as usize;
            let dst_x = y;
            let dst_y = original_width - 1 - x;
            let dst_idx = ((dst_y * original_height + dst_x) * 4) as usize;
            
            rotated_data[dst_idx..dst_idx + 4].copy_from_slice(&original_data[src_idx..src_idx + 4]);
        }
    }
    
    *image = ImageBuffer::from_raw(original_height, original_width, rotated_data).unwrap();
}

fn rotate_180(image: &mut ImageData) {
    let width = image.width();
    let height = image.height();
    let total_pixels = (width * height) as usize;
    let data = image.as_mut();
    
    for i in 0..total_pixels / 2 {
        let front_idx = i * 4;
        let back_idx = (total_pixels - 1 - i) * 4;
        
        for j in 0..4 {
            data.swap(front_idx + j, back_idx + j);
        }
    }
}

// Bilinear interpolation for smooth rotation
fn bilinear_interpolate(image: &ImageData, x: f32, y: f32) -> [u8; 4] {
    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let x1 = (x0 + 1).min(image.width() - 1);
    let y1 = (y0 + 1).min(image.height() - 1);
    
    let fx = x - x0 as f32;
    let fy = y - y0 as f32;
    
    let p00 = image.get_pixel(x0, y0);
    let p10 = image.get_pixel(x1, y0);
    let p01 = image.get_pixel(x0, y1);
    let p11 = image.get_pixel(x1, y1);
    
    let mut result = [0u8; 4];
    
    for i in 0..4 {
        let a = p00[i] as f32 * (1.0 - fx) + p10[i] as f32 * fx;
        let b = p01[i] as f32 * (1.0 - fx) + p11[i] as f32 * fx;
        result[i] = (a * (1.0 - fy) + b * fy).round().clamp(0.0, 255.0) as u8;
    }
    
    result
}
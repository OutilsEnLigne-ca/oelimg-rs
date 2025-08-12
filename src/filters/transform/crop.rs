use crate::filters::{ImageFilter, ImageData};
use crate::utils::validate_image_size;
use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CropFilter {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl CropFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Result<CropFilter, JsValue> {
        validate_image_size(width, height)?;
        
        Ok(CropFilter {
            x,
            y,
            width,
            height,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    // Create crop filter from percentage values (0.0 - 1.0)
    pub fn from_percentage(
        image_width: u32,
        image_height: u32,
        x_percent: f32,
        y_percent: f32,
        width_percent: f32,
        height_percent: f32,
    ) -> Result<CropFilter, JsValue> {
        let x = (image_width as f32 * x_percent.max(0.0).min(1.0)) as u32;
        let y = (image_height as f32 * y_percent.max(0.0).min(1.0)) as u32;
        let width = (image_width as f32 * width_percent.max(0.0).min(1.0)) as u32;
        let height = (image_height as f32 * height_percent.max(0.0).min(1.0)) as u32;
        
        Self::new(x, y, width, height)
    }

    // Center crop to specific aspect ratio
    pub fn center_crop_aspect_ratio(
        image_width: u32,
        image_height: u32,
        aspect_width: u32,
        aspect_height: u32,
    ) -> Result<CropFilter, JsValue> {
        let target_aspect = aspect_width as f32 / aspect_height as f32;
        let image_aspect = image_width as f32 / image_height as f32;
        
        let (crop_width, crop_height) = if image_aspect > target_aspect {
            // Image is wider than target aspect - crop width
            let crop_width = (image_height as f32 * target_aspect) as u32;
            (crop_width, image_height)
        } else {
            // Image is taller than target aspect - crop height
            let crop_height = (image_width as f32 / target_aspect) as u32;
            (image_width, crop_height)
        };
        
        let x = (image_width - crop_width) / 2;
        let y = (image_height - crop_height) / 2;
        
        Self::new(x, y, crop_width, crop_height)
    }

    // Square crop from center
    pub fn center_square(image_width: u32, image_height: u32) -> Result<CropFilter, JsValue> {
        Self::center_crop_aspect_ratio(image_width, image_height, 1, 1)
    }
}

impl ImageFilter for CropFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let img_width = image.width();
        let img_height = image.height();
        
        // Validate crop boundaries
        if self.x >= img_width || self.y >= img_height {
            return Err(JsValue::from_str("Crop start position is outside image bounds"));
        }
        
        // Adjust crop dimensions if they exceed image boundaries
        let actual_width = self.width.min(img_width - self.x);
        let actual_height = self.height.min(img_height - self.y);
        
        if actual_width == 0 || actual_height == 0 {
            return Err(JsValue::from_str("Invalid crop dimensions"));
        }
        
        // Create new image buffer for cropped result
        let mut cropped_data = Vec::with_capacity((actual_width * actual_height * 4) as usize);
        
        // Copy pixels from source to cropped image
        for y in 0..actual_height {
            for x in 0..actual_width {
                let src_x = self.x + x;
                let src_y = self.y + y;
                let pixel = image.get_pixel(src_x, src_y);
                
                cropped_data.push(pixel[0]); // R
                cropped_data.push(pixel[1]); // G
                cropped_data.push(pixel[2]); // B
                cropped_data.push(pixel[3]); // A
            }
        }
        
        // Replace the image data with cropped data
        *image = ImageBuffer::from_raw(actual_width, actual_height, cropped_data)
            .ok_or_else(|| JsValue::from_str("Failed to create cropped image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Crop"
    }
}

// Smart crop filter that tries to find the most interesting part of the image
#[wasm_bindgen]
pub struct SmartCropFilter {
    target_width: u32,
    target_height: u32,
}

#[wasm_bindgen]
impl SmartCropFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(target_width: u32, target_height: u32) -> Result<SmartCropFilter, JsValue> {
        validate_image_size(target_width, target_height)?;
        
        Ok(SmartCropFilter {
            target_width,
            target_height,
        })
    }
}

impl ImageFilter for SmartCropFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let img_width = image.width();
        let img_height = image.height();
        
        // If target dimensions are larger than image, return error
        if self.target_width > img_width || self.target_height > img_height {
            return Err(JsValue::from_str("Target crop size is larger than source image"));
        }
        
        // Simple smart cropping: find the area with highest contrast/detail
        let mut best_score = 0.0;
        let mut best_x = 0;
        let mut best_y = 0;
        
        let step = 8; // Sample every 8 pixels for performance
        let max_x = img_width - self.target_width;
        let max_y = img_height - self.target_height;
        
        for y in (0..=max_y).step_by(step as usize) {
            for x in (0..=max_x).step_by(step as usize) {
                let score = calculate_crop_score(image, x, y, self.target_width, self.target_height);
                if score > best_score {
                    best_score = score;
                    best_x = x;
                    best_y = y;
                }
            }
        }
        
        // Apply the best crop
        let crop_filter = CropFilter::new(best_x, best_y, self.target_width, self.target_height)?;
        crop_filter.apply(image)
    }

    fn name(&self) -> &'static str {
        "Smart Crop"
    }
}

// Helper function to calculate how interesting a crop region is
fn calculate_crop_score(image: &ImageData, x: u32, y: u32, width: u32, height: u32) -> f32 {
    let mut total_gradient = 0.0;
    let mut pixel_count = 0;
    
    let sample_step = 4; // Sample every 4 pixels for performance
    
    for py in (y..y + height).step_by(sample_step as usize) {
        for px in (x..x + width).step_by(sample_step as usize) {
            if px + 1 < image.width() && py + 1 < image.height() {
                let current = image.get_pixel(px, py);
                let right = image.get_pixel(px + 1, py);
                let bottom = image.get_pixel(px, py + 1);
                
                // Calculate gradients
                let grad_x = ((right[0] as i16 - current[0] as i16).abs() +
                             (right[1] as i16 - current[1] as i16).abs() +
                             (right[2] as i16 - current[2] as i16).abs()) as f32;
                             
                let grad_y = ((bottom[0] as i16 - current[0] as i16).abs() +
                             (bottom[1] as i16 - current[1] as i16).abs() +
                             (bottom[2] as i16 - current[2] as i16).abs()) as f32;
                
                total_gradient += (grad_x + grad_y) / 3.0; // Average RGB gradient
                pixel_count += 1;
            }
        }
    }
    
    if pixel_count > 0 {
        total_gradient / pixel_count as f32
    } else {
        0.0
    }
}
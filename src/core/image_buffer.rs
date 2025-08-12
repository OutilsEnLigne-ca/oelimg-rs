use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;

pub type ImageData = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[wasm_bindgen]
pub struct ImageProcessor {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl ImageProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Result<ImageProcessor, JsValue> {
        crate::utils::validate_image_size(width, height)?;
        
        let expected_len = (width * height * 4) as usize;
        if data.len() != expected_len {
            return Err(JsValue::from_str(&format!(
                "Invalid data length. Expected {}, got {}",
                expected_len,
                data.len()
            )));
        }
        
        Ok(ImageProcessor {
            width,
            height,
            data,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Vec<u8>, JsValue> {
        if x >= self.width || y >= self.height {
            return Err(JsValue::from_str("Pixel coordinates out of bounds"));
        }
        
        let idx = ((y * self.width + x) * 4) as usize;
        Ok(self.data[idx..idx + 4].to_vec())
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgba: Vec<u8>) -> Result<(), JsValue> {
        if x >= self.width || y >= self.height {
            return Err(JsValue::from_str("Pixel coordinates out of bounds"));
        }
        
        if rgba.len() != 4 {
            return Err(JsValue::from_str("RGBA array must have exactly 4 elements"));
        }
        
        let idx = ((y * self.width + x) * 4) as usize;
        self.data[idx..idx + 4].copy_from_slice(&rgba);
        Ok(())
    }
}

impl ImageProcessor {
    pub fn to_image_buffer(&self) -> ImageData {
        ImageBuffer::from_raw(self.width, self.height, self.data.clone())
            .expect("Failed to create ImageBuffer from raw data")
    }

    pub fn from_image_buffer(image: &ImageData) -> Self {
        ImageProcessor {
            width: image.width(),
            height: image.height(),
            data: image.as_raw().clone(),
        }
    }

    pub fn update_from_image_buffer(&mut self, image: &ImageData) {
        self.width = image.width();
        self.height = image.height();
        self.data = image.as_raw().clone();
    }
}
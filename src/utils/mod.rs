use wasm_bindgen::prelude::*;
use image::{ImageBuffer, Rgba, ImageFormat};

pub mod svg;

pub type ImageData = ImageBuffer<Rgba<u8>, Vec<u8>>;

// Utility functions for image format conversion and validation
pub fn bytes_to_image(bytes: &[u8]) -> Result<ImageData, JsValue> {
    // Check if it's an SVG file first
    if svg::is_svg_bytes(bytes) {
        return bytes_to_image_from_svg(bytes, None, None);
    }
    
    // Otherwise, try to load as regular raster image
    let img = image::load_from_memory(bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;
    
    Ok(img.to_rgba8())
}

// Specialized function for SVG to image conversion with size options
pub fn bytes_to_image_from_svg(
    bytes: &[u8], 
    width: Option<u32>, 
    height: Option<u32>
) -> Result<ImageData, JsValue> {
    use crate::core::svg_renderer::{SvgRenderer, SvgRenderOptions};
    
    let renderer = SvgRenderer::new();
    let options = SvgRenderOptions {
        width,
        height,
        ..Default::default()
    };
    
    renderer.render_svg_to_image(bytes, options)
}

// Function to convert SVG with background color
pub fn bytes_to_image_from_svg_with_background(
    bytes: &[u8], 
    width: Option<u32>, 
    height: Option<u32>,
    background_color: [u8; 4]
) -> Result<ImageData, JsValue> {
    use crate::core::svg_renderer::{SvgRenderer, SvgRenderOptions};
    
    let renderer = SvgRenderer::new();
    let options = SvgRenderOptions {
        width,
        height,
        background_color: Some(background_color),
        ..Default::default()
    };
    
    renderer.render_svg_to_image(bytes, options)
}

pub fn image_to_bytes(image: &ImageData, format: ImageFormat) -> Result<Vec<u8>, JsValue> {
    let mut bytes = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);
    
    image.write_to(&mut cursor, format)
        .map_err(|e| JsValue::from_str(&format!("Failed to encode image: {}", e)))?;
    
    Ok(bytes)
}

// Validate image dimensions and format
pub fn validate_image_size(width: u32, height: u32) -> Result<(), JsValue> {
    const MAX_DIMENSION: u32 = 8192;
    const MAX_PIXELS: u64 = 64_000_000; // ~64MP
    
    if width == 0 || height == 0 {
        return Err(JsValue::from_str("Image dimensions must be greater than 0"));
    }
    
    if width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Err(JsValue::from_str(&format!(
            "Image dimensions too large. Maximum: {}x{}", 
            MAX_DIMENSION, MAX_DIMENSION
        )));
    }
    
    let total_pixels = width as u64 * height as u64;
    if total_pixels > MAX_PIXELS {
        return Err(JsValue::from_str(&format!(
            "Image too large. Maximum pixels: {}", 
            MAX_PIXELS
        )));
    }
    
    Ok(())
}

// Clamp values to valid ranges
pub fn clamp_f32(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

pub fn clamp_u8(value: i32) -> u8 {
    value.max(0).min(255) as u8
}
use wasm_bindgen::prelude::*;
use image::ImageFormat;
use crate::core::ImageProcessor;
use crate::filters::*;
use crate::utils;

// Set up console logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Main image processing class for WebAssembly
/// 
/// This class provides a comprehensive set of image processing operations
/// including color adjustments, transforms, artistic filters, and correction tools.
/// All operations are performed in-place for memory efficiency.
/// 
/// # Example
/// 
/// ```javascript
/// import init, { WasmImageProcessor } from 'oelimg-rs';
/// 
/// await init();
/// const processor = WasmImageProcessor.from_bytes(imageBytes);
/// processor.gaussian_blur(2.0);
/// processor.adjust_brightness_contrast(0.1, 1.2);
/// const result = processor.to_png_bytes();
/// ```
#[wasm_bindgen]
pub struct WasmImageProcessor {
    processor: ImageProcessor,
}

#[wasm_bindgen]
impl WasmImageProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Result<WasmImageProcessor, JsValue> {
        console_error_panic_hook::set_once();
        
        let processor = ImageProcessor::new(width, height, data)?;
        Ok(WasmImageProcessor { processor })
    }

    #[wasm_bindgen]
    pub fn from_bytes(bytes: &[u8]) -> Result<WasmImageProcessor, JsValue> {
        console_error_panic_hook::set_once();
        
        let image = utils::bytes_to_image(bytes)?;
        let processor = ImageProcessor::from_image_buffer(&image);
        
        Ok(WasmImageProcessor { processor })
    }

    /// Create processor from SVG bytes with specific dimensions
    /// 
    /// # Parameters
    /// - `bytes`: SVG file content as bytes
    /// - `width`: Target width (optional, will auto-calculate if not provided)
    /// - `height`: Target height (optional, will auto-calculate if not provided)  
    /// - `background_color`: Background color as RGBA array (optional, transparent by default)
    /// 
    /// # Example
    /// ```javascript
    /// // Auto-size based on SVG dimensions
    /// const processor = WasmImageProcessor.from_svg_bytes(svgBytes);
    /// 
    /// // Specific size
    /// const processor = WasmImageProcessor.from_svg_bytes(svgBytes, 400, 300);
    /// 
    /// // With white background  
    /// const processor = WasmImageProcessor.from_svg_bytes(svgBytes, 400, 300, [255, 255, 255, 255]);
    /// ```
    #[wasm_bindgen]
    pub fn from_svg_bytes(
        bytes: &[u8], 
        width: Option<u32>, 
        height: Option<u32>,
        background_color: Option<Vec<u8>>
    ) -> Result<WasmImageProcessor, JsValue> {
        console_error_panic_hook::set_once();
        
        let image = if let Some(bg_color) = background_color {
            if bg_color.len() != 4 {
                return Err(JsValue::from_str("Background color must be RGBA array with 4 elements"));
            }
            let bg_array: [u8; 4] = [bg_color[0], bg_color[1], bg_color[2], bg_color[3]];
            utils::bytes_to_image_from_svg_with_background(bytes, width, height, bg_array)?
        } else {
            utils::bytes_to_image_from_svg(bytes, width, height)?
        };
        
        let processor = ImageProcessor::from_image_buffer(&image);
        
        Ok(WasmImageProcessor { processor })
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.processor.width()
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.processor.height()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.processor.data()
    }

    #[wasm_bindgen]
    pub fn to_png_bytes(&self) -> Result<Vec<u8>, JsValue> {
        let image = self.processor.to_image_buffer();
        utils::image_to_bytes(&image, ImageFormat::Png)
    }

    #[wasm_bindgen]
    pub fn to_jpeg_bytes(&self, quality: u8) -> Result<Vec<u8>, JsValue> {
        let image = self.processor.to_image_buffer();
        // Note: quality parameter would need custom JPEG encoder integration
        utils::image_to_bytes(&image, ImageFormat::Jpeg)
    }

    #[wasm_bindgen]
    pub fn to_webp_bytes(&self) -> Result<Vec<u8>, JsValue> {
        let image = self.processor.to_image_buffer();
        utils::image_to_bytes(&image, ImageFormat::WebP)
    }

    #[wasm_bindgen]
    pub fn to_ico_bytes(&self) -> Result<Vec<u8>, JsValue> {
        let image = self.processor.to_image_buffer();
        utils::image_to_bytes(&image, ImageFormat::Ico)
    }

    /// Adjust hue, saturation, and lightness values
    /// 
    /// # Parameters
    /// - `hue`: Hue shift in degrees (-180 to 180)
    /// - `saturation`: Saturation multiplier (0.0 to 2.0, 1.0 = no change)
    /// - `lightness`: Lightness multiplier (0.0 to 2.0, 1.0 = no change)
    /// 
    /// # Example
    /// ```javascript
    /// processor.adjust_hsl(10, 1.2, 1.0); // +10° hue, +20% saturation
    /// ```
    #[wasm_bindgen]
    pub fn adjust_hsl(&mut self, hue: f32, saturation: f32, lightness: f32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = color::HslFilter::new(hue, saturation, lightness);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn adjust_brightness_contrast(&mut self, brightness: f32, contrast: f32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = color::BrightnessContrastFilter::new(brightness, contrast);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn adjust_color_temperature(&mut self, kelvin: f32, strength: Option<f32>) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = color::ColorTemperatureFilter::new(kelvin, strength);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn adjust_vibrance(&mut self, vibrance: f32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = color::VibranceFilter::new(vibrance);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    // Transform methods
    #[wasm_bindgen]
    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = transform::CropFilter::new(x, y, width, height)?;
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32, algorithm: Option<transform::ResizeAlgorithm>) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = transform::ResizeFilter::new(width, height, algorithm, Some(false))?;
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn resize_fit(&mut self, max_width: u32, max_height: u32, algorithm: Option<transform::ResizeAlgorithm>) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = transform::ResizeFilter::fit(max_width, max_height, algorithm)?;
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn rotate(&mut self, angle_degrees: f32, background_color: Option<Vec<u8>>) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = transform::RotateFilter::new(angle_degrees, background_color)?;
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn flip_horizontal(&mut self) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = transform::FlipFilter::horizontal();
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn flip_vertical(&mut self) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = transform::FlipFilter::vertical();
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    // Artistic filters
    #[wasm_bindgen]
    pub fn gaussian_blur(&mut self, radius: f32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = artistic::GaussianBlurFilter::new(radius);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn sharpen(&mut self, strength: f32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = artistic::SharpenFilter::new(strength);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn sepia(&mut self, intensity: f32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = artistic::SepiaFilter::new(intensity);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn vintage(&mut self, warmth: f32, vignette: f32, grain: f32, fade: f32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = artistic::VintageFilter::new(warmth, vignette, grain, fade);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    // Correction filters
    #[wasm_bindgen]
    pub fn auto_levels(&mut self) -> Result<(), JsValue> {
        let image = self.processor.to_image_buffer();
        let filter = correction::LevelsFilter::auto_levels_from_image(&image);
        let mut image = image; // Make mutable
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn adjust_levels(&mut self, shadows: f32, gamma: f32, highlights: f32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = correction::LevelsFilter::new(shadows, gamma, highlights, 0.0, 1.0);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn adjust_exposure(&mut self, exposure: f32, highlights: f32, shadows: f32) -> Result<(), JsValue> {
        let mut image = self.processor.to_image_buffer();
        let filter = correction::ExposureFilter::new(exposure, highlights, shadows);
        filter.apply(&mut image)?;
        self.processor.update_from_image_buffer(&image);
        Ok(())
    }
}

// Utility functions for working with image data
#[wasm_bindgen]
pub fn load_image_from_bytes(bytes: &[u8]) -> Result<WasmImageProcessor, JsValue> {
    WasmImageProcessor::from_bytes(bytes)
}

/// Convert SVG to raster image and return as PNG bytes
#[wasm_bindgen]
pub fn convert_svg_to_png(
    svg_bytes: &[u8],
    width: Option<u32>,
    height: Option<u32>,
    background_color: Option<Vec<u8>>
) -> Result<Vec<u8>, JsValue> {
    let processor = WasmImageProcessor::from_svg_bytes(svg_bytes, width, height, background_color)?;
    processor.to_png_bytes()
}

/// Convert SVG to raster image and return as WebP bytes
#[wasm_bindgen]
pub fn convert_svg_to_webp(
    svg_bytes: &[u8],
    width: Option<u32>,
    height: Option<u32>,
    background_color: Option<Vec<u8>>
) -> Result<Vec<u8>, JsValue> {
    let processor = WasmImageProcessor::from_svg_bytes(svg_bytes, width, height, background_color)?;
    processor.to_webp_bytes()
}

/// Check if the provided bytes contain SVG content
#[wasm_bindgen]
pub fn is_svg_format(bytes: &[u8]) -> bool {
    crate::utils::svg::is_svg_bytes(bytes)
}

/// Convert any image format to ICO format
#[wasm_bindgen]
pub fn convert_to_ico(bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let processor = WasmImageProcessor::from_bytes(bytes)?;
    processor.to_ico_bytes()
}

/// Create a favicon from image bytes, optimized for web usage
/// 
/// Generates a 32x32 ICO file which is the standard favicon size
/// Applies optimization for small icon display (slight sharpening)
/// 
/// # Parameters
/// - `bytes: &[u8]` - Source image data (any supported format)
/// 
/// # Returns
/// ICO file bytes ready for web deployment as favicon.ico
/// 
/// # Example
/// ```javascript
/// const faviconBytes = create_favicon(imageBytes);
/// // Save as favicon.ico or serve directly
/// ```
#[wasm_bindgen]
pub fn create_favicon(bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut processor = WasmImageProcessor::from_bytes(bytes)?;
    
    // Resize to standard favicon size (32x32)
    // Use Lanczos3 for best quality when downscaling
    processor.resize(32, 32, Some(transform::ResizeAlgorithm::Lanczos3))?;
    
    // Apply slight sharpening to compensate for small size display
    processor.sharpen(0.3)?;
    
    // Convert to ICO format
    processor.to_ico_bytes()
}

#[wasm_bindgen]
pub fn create_thumbnail(
    bytes: &[u8],
    size: u32,
    crop_to_square: Option<bool>
) -> Result<Vec<u8>, JsValue> {
    let mut processor = WasmImageProcessor::from_bytes(bytes)?;
    let mut image = processor.processor.to_image_buffer();
    
    let filter = transform::ThumbnailFilter::new(
        size,
        Some(transform::ResizeAlgorithm::Lanczos3),
        crop_to_square
    )?;
    
    filter.apply(&mut image)?;
    utils::image_to_bytes(&image, ImageFormat::Png)
}

// Filter preset functions
#[wasm_bindgen]
pub fn apply_instagram_filter(bytes: &[u8], filter_name: &str) -> Result<Vec<u8>, JsValue> {
    let mut processor = WasmImageProcessor::from_bytes(bytes)?;
    let mut image = processor.processor.to_image_buffer();
    
    match filter_name {
        "clarendon" => {
            // High contrast with lifted shadows
            let levels = correction::LevelsFilter::new(0.1, 0.9, 0.95, 0.0, 1.0);
            levels.apply(&mut image)?;
            let vibrance = color::VibranceFilter::new(15.0);
            vibrance.apply(&mut image)?;
        },
        "gingham" => {
            // Soft, dreamy look
            let brightness = color::BrightnessFilter::new(0.05);
            brightness.apply(&mut image)?;
            let vintage = artistic::VintageFilter::new(1.1, 0.2, 0.1, 0.1);
            vintage.apply(&mut image)?;
        },
        "moon" => {
            // Desaturated, cool tone
            let saturation = color::SaturationFilter::new(0.7);
            saturation.apply(&mut image)?;
            let temp = color::ColorTemperatureFilter::new(7000.0, Some(0.3));
            temp.apply(&mut image)?;
        },
        "lark" => {
            // Bright and vibrant
            let brightness = color::BrightnessFilter::new(0.1);
            brightness.apply(&mut image)?;
            let vibrance = color::VibranceFilter::new(20.0);
            vibrance.apply(&mut image)?;
        },
        "reyes" => {
            // Vintage film look
            let vintage = artistic::VintageFilter::new(1.2, 0.3, 0.2, 0.2);
            vintage.apply(&mut image)?;
            let sepia = artistic::SepiaFilter::new(0.1);
            sepia.apply(&mut image)?;
        },
        _ => {
            return Err(JsValue::from_str("Unknown filter name"));
        }
    }
    
    utils::image_to_bytes(&image, ImageFormat::Png)
}

// Version information
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn get_supported_formats() -> Vec<String> {
    vec![
        "PNG".to_string(),
        "JPEG".to_string(), 
        "WebP".to_string(),
        "ICO".to_string(),
        "SVG".to_string(),
    ]
}

// Performance test function
#[wasm_bindgen]
pub fn benchmark_filters(bytes: &[u8], iterations: u32) -> Result<String, JsValue> {
    use std::time::Instant;
    
    let mut results = Vec::new();
    
    // Test basic operations
    let start = Instant::now();
    for _ in 0..iterations {
        let _processor = WasmImageProcessor::from_bytes(bytes)?;
    }
    let load_time = start.elapsed();
    results.push(format!("Load: {:.2}ms", load_time.as_millis() as f64 / iterations as f64));
    
    // Test blur
    let start = Instant::now();
    for _ in 0..iterations {
        let mut processor = WasmImageProcessor::from_bytes(bytes)?;
        processor.gaussian_blur(2.0)?;
    }
    let blur_time = start.elapsed();
    results.push(format!("Blur: {:.2}ms", blur_time.as_millis() as f64 / iterations as f64));
    
    // Test resize
    let start = Instant::now();
    for _ in 0..iterations {
        let mut processor = WasmImageProcessor::from_bytes(bytes)?;
        processor.resize_fit(512, 512, None)?;
    }
    let resize_time = start.elapsed();
    results.push(format!("Resize: {:.2}ms", resize_time.as_millis() as f64 / iterations as f64));
    
    Ok(results.join(", "))
}
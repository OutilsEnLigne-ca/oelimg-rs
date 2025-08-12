use crate::filters::{ImageFilter, ImageData};
use crate::utils::validate_image_size;
use fast_image_resize as fir;
use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ResizeAlgorithm {
    Nearest = "Nearest",
    Bilinear = "Bilinear",
    CatmullRom = "CatmullRom",
    Mitchell = "Mitchell",
    Lanczos3 = "Lanczos3",
}

impl From<ResizeAlgorithm> for fir::ResizeAlg {
    fn from(alg: ResizeAlgorithm) -> Self {
        match alg {
            ResizeAlgorithm::Nearest => fir::ResizeAlg::Nearest,
            ResizeAlgorithm::Bilinear => fir::ResizeAlg::Convolution(fir::FilterType::Bilinear),
            ResizeAlgorithm::CatmullRom => fir::ResizeAlg::Convolution(fir::FilterType::CatmullRom),
            ResizeAlgorithm::Mitchell => fir::ResizeAlg::Convolution(fir::FilterType::Mitchell),
            ResizeAlgorithm::Lanczos3 => fir::ResizeAlg::Convolution(fir::FilterType::Lanczos3),
            _ => fir::ResizeAlg::Convolution(fir::FilterType::Lanczos3), // Default fallback
        }
    }
}

#[wasm_bindgen]
pub struct ResizeFilter {
    width: u32,
    height: u32,
    algorithm: ResizeAlgorithm,
    maintain_aspect_ratio: bool,
}

#[wasm_bindgen]
impl ResizeFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: u32,
        height: u32,
        algorithm: Option<ResizeAlgorithm>,
        maintain_aspect_ratio: Option<bool>,
    ) -> Result<ResizeFilter, JsValue> {
        validate_image_size(width, height)?;
        
        Ok(ResizeFilter {
            width,
            height,
            algorithm: algorithm.unwrap_or(ResizeAlgorithm::Lanczos3),
            maintain_aspect_ratio: maintain_aspect_ratio.unwrap_or(false),
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
    pub fn algorithm(&self) -> ResizeAlgorithm {
        self.algorithm
    }

    #[wasm_bindgen(getter)]
    pub fn maintain_aspect_ratio(&self) -> bool {
        self.maintain_aspect_ratio
    }

    // Create resize filter that fits image within bounds while maintaining aspect ratio
    pub fn fit(
        max_width: u32,
        max_height: u32,
        algorithm: Option<ResizeAlgorithm>,
    ) -> Result<ResizeFilter, JsValue> {
        validate_image_size(max_width, max_height)?;
        
        Ok(ResizeFilter {
            width: max_width,
            height: max_height,
            algorithm: algorithm.unwrap_or(ResizeAlgorithm::Lanczos3),
            maintain_aspect_ratio: true,
        })
    }

    // Calculate actual dimensions considering aspect ratio
    fn calculate_dimensions(&self, original_width: u32, original_height: u32) -> (u32, u32) {
        if !self.maintain_aspect_ratio {
            return (self.width, self.height);
        }

        let original_aspect = original_width as f32 / original_height as f32;
        let target_aspect = self.width as f32 / self.height as f32;

        if original_aspect > target_aspect {
            // Original is wider - fit to width
            let new_height = (self.width as f32 / original_aspect) as u32;
            (self.width, new_height.max(1))
        } else {
            // Original is taller - fit to height
            let new_width = (self.height as f32 * original_aspect) as u32;
            (new_width.max(1), self.height)
        }
    }
}

impl ImageFilter for ResizeFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let original_width = image.width();
        let original_height = image.height();
        
        // Calculate target dimensions
        let (target_width, target_height) = self.calculate_dimensions(original_width, original_height);
        
        // If dimensions are the same, no need to resize
        if target_width == original_width && target_height == original_height {
            return Ok(());
        }
        
        // Create fast_image_resize images
        let src_image = fir::Image::from_vec_u8(
            std::num::NonZeroU32::new(original_width).unwrap(),
            std::num::NonZeroU32::new(original_height).unwrap(),
            image.as_raw().clone(),
            fir::PixelType::U8x4,
        ).map_err(|e| JsValue::from_str(&format!("Failed to create source image: {}", e)))?;
        
        let mut dst_image = fir::Image::new(
            std::num::NonZeroU32::new(target_width).unwrap(),
            std::num::NonZeroU32::new(target_height).unwrap(),
            fir::PixelType::U8x4,
        );
        
        // Create resizer
        let mut resizer = fir::Resizer::new(
            self.algorithm.into()
        );
        
        // Perform resize
        resizer
            .resize(&src_image.view(), &mut dst_image.view_mut())
            .map_err(|e| JsValue::from_str(&format!("Resize failed: {}", e)))?;
        
        // Replace the image data with resized data
        *image = ImageBuffer::from_raw(target_width, target_height, dst_image.into_vec())
            .ok_or_else(|| JsValue::from_str("Failed to create resized image buffer"))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Resize"
    }
}

// Scale filter for percentage-based resizing
#[wasm_bindgen]
pub struct ScaleFilter {
    scale_x: f32,
    scale_y: f32,
    algorithm: ResizeAlgorithm,
}

#[wasm_bindgen]
impl ScaleFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        scale_x: f32,
        scale_y: f32,
        algorithm: Option<ResizeAlgorithm>,
    ) -> Result<ScaleFilter, JsValue> {
        if scale_x <= 0.0 || scale_y <= 0.0 {
            return Err(JsValue::from_str("Scale factors must be positive"));
        }
        
        if scale_x > 10.0 || scale_y > 10.0 {
            return Err(JsValue::from_str("Scale factors too large (max 10.0)"));
        }
        
        Ok(ScaleFilter {
            scale_x,
            scale_y,
            algorithm: algorithm.unwrap_or(ResizeAlgorithm::Lanczos3),
        })
    }

    // Uniform scaling
    pub fn uniform(scale: f32, algorithm: Option<ResizeAlgorithm>) -> Result<ScaleFilter, JsValue> {
        Self::new(scale, scale, algorithm)
    }

    #[wasm_bindgen(getter)]
    pub fn scale_x(&self) -> f32 {
        self.scale_x
    }

    #[wasm_bindgen(getter)]
    pub fn scale_y(&self) -> f32 {
        self.scale_y
    }

    #[wasm_bindgen(getter)]
    pub fn algorithm(&self) -> ResizeAlgorithm {
        self.algorithm
    }
}

impl ImageFilter for ScaleFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let original_width = image.width();
        let original_height = image.height();
        
        let target_width = (original_width as f32 * self.scale_x).round() as u32;
        let target_height = (original_height as f32 * self.scale_y).round() as u32;
        
        // Validate target dimensions
        validate_image_size(target_width, target_height)?;
        
        // Create and apply resize filter
        let resize_filter = ResizeFilter {
            width: target_width,
            height: target_height,
            algorithm: self.algorithm,
            maintain_aspect_ratio: false,
        };
        
        resize_filter.apply(image)
    }

    fn name(&self) -> &'static str {
        "Scale"
    }
}

// Thumbnail generator with smart cropping and resizing
#[wasm_bindgen]
pub struct ThumbnailFilter {
    size: u32,
    algorithm: ResizeAlgorithm,
    crop_to_square: bool,
}

#[wasm_bindgen]
impl ThumbnailFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        size: u32,
        algorithm: Option<ResizeAlgorithm>,
        crop_to_square: Option<bool>,
    ) -> Result<ThumbnailFilter, JsValue> {
        validate_image_size(size, size)?;
        
        if size > 1024 {
            return Err(JsValue::from_str("Thumbnail size too large (max 1024)"));
        }
        
        Ok(ThumbnailFilter {
            size,
            algorithm: algorithm.unwrap_or(ResizeAlgorithm::Lanczos3),
            crop_to_square: crop_to_square.unwrap_or(true),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> u32 {
        self.size
    }

    #[wasm_bindgen(getter)]
    pub fn algorithm(&self) -> ResizeAlgorithm {
        self.algorithm
    }

    #[wasm_bindgen(getter)]
    pub fn crop_to_square(&self) -> bool {
        self.crop_to_square
    }
}

impl ImageFilter for ThumbnailFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let original_width = image.width();
        let original_height = image.height();
        
        if self.crop_to_square {
            // First crop to square from center
            let crop_filter = crate::filters::transform::CropFilter::center_square(original_width, original_height)?;
            crop_filter.apply(image)?;
        }
        
        // Then resize to target size
        let resize_filter = if self.crop_to_square {
            ResizeFilter::new(self.size, self.size, Some(self.algorithm), Some(false))?
        } else {
            ResizeFilter::fit(self.size, self.size, Some(self.algorithm))?
        };
        
        resize_filter.apply(image)
    }

    fn name(&self) -> &'static str {
        "Thumbnail"
    }
}
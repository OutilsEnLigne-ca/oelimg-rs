use wasm_bindgen::prelude::*;
use image::ImageBuffer;
use crate::utils::ImageData;
use crate::utils::svg::validate_svg_content;
use usvg::{TreeParsing, TreeTextToPath};

pub struct SvgRenderer {
    default_width: u32,
    default_height: u32,
    background_color: tiny_skia::Color,
}

#[derive(Debug, Clone)]
pub struct SvgRenderOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub dpi: f32,
    pub background_color: Option<[u8; 4]>, // RGBA
    pub preserve_aspect_ratio: bool,
}

impl Default for SvgRenderOptions {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            dpi: 96.0, // Standard web DPI
            background_color: None, // Transparent by default
            preserve_aspect_ratio: true,
        }
    }
}

impl SvgRenderer {
    pub fn new() -> Self {
        Self {
            default_width: 800,
            default_height: 600,
            background_color: tiny_skia::Color::TRANSPARENT,
        }
    }
    
    pub fn with_defaults(width: u32, height: u32) -> Self {
        Self {
            default_width: width,
            default_height: height,
            background_color: tiny_skia::Color::TRANSPARENT,
        }
    }
    
    pub fn render_svg_to_image(
        &self, 
        svg_bytes: &[u8], 
        options: SvgRenderOptions
    ) -> Result<ImageData, JsValue> {
        // Validate SVG content first
        validate_svg_content(svg_bytes)?;
        
        // Parse SVG with usvg
        let usvg_tree = usvg::Tree::from_data(svg_bytes, &usvg::Options::default())
            .map_err(|e| JsValue::from_str(&format!("SVG parsing failed: {}", e)))?;
        
        // Convert text to paths for rendering
        let mut usvg_tree = usvg_tree;
        usvg_tree.convert_text(&fontdb::Database::new());
        
        // Create resvg tree from usvg tree
        let resvg_tree = resvg::Tree::from_usvg(&usvg_tree);
        
        // Determine render dimensions
        let (render_width, render_height) = self.calculate_render_dimensions(&usvg_tree, &options)?;
        
        // Create tiny-skia pixmap
        let mut pixmap = tiny_skia::Pixmap::new(render_width, render_height)
            .ok_or_else(|| JsValue::from_str("Failed to create rendering surface"))?;
        
        // Set background color if specified
        if let Some(bg_color) = options.background_color {
            let bg = tiny_skia::Color::from_rgba8(bg_color[0], bg_color[1], bg_color[2], bg_color[3]);
            pixmap.fill(bg);
        }
        
        // Calculate scale transform if needed
        let svg_size = usvg_tree.size;
        let scale_x = render_width as f32 / svg_size.width();
        let scale_y = render_height as f32 / svg_size.height();
        
        let scale = if options.preserve_aspect_ratio {
            scale_x.min(scale_y)
        } else {
            // Use different scales for x and y (stretch to fit)
            1.0 // We'll use transform matrix instead
        };
        
        let transform = if options.preserve_aspect_ratio {
            // Center the SVG if preserving aspect ratio
            let scaled_width = svg_size.width() * scale;
            let scaled_height = svg_size.height() * scale;
            let offset_x = (render_width as f32 - scaled_width) / 2.0;
            let offset_y = (render_height as f32 - scaled_height) / 2.0;
            
            tiny_skia::Transform::from_translate(offset_x, offset_y)
                .post_scale(scale, scale)
        } else {
            tiny_skia::Transform::from_scale(scale_x, scale_y)
        };
        
        // Render SVG using resvg Tree
        resvg_tree.render(transform, &mut pixmap.as_mut());
        
        // Convert tiny-skia pixmap to image::ImageBuffer
        self.pixmap_to_image_buffer(pixmap)
    }
    
    fn calculate_render_dimensions(
        &self, 
        usvg_tree: &usvg::Tree, 
        options: &SvgRenderOptions
    ) -> Result<(u32, u32), JsValue> {
        let svg_size = usvg_tree.size;
        
        match (options.width, options.height) {
            // Both specified
            (Some(w), Some(h)) => Ok((w, h)),
            
            // Only width specified, calculate height preserving aspect ratio
            (Some(w), None) => {
                let aspect_ratio = svg_size.height() / svg_size.width();
                let h = (w as f32 * aspect_ratio).round() as u32;
                Ok((w, h))
            }
            
            // Only height specified, calculate width preserving aspect ratio  
            (None, Some(h)) => {
                let aspect_ratio = svg_size.width() / svg_size.height();
                let w = (h as f32 * aspect_ratio).round() as u32;
                Ok((w, h))
            }
            
            // Neither specified, use SVG's intrinsic size or defaults
            (None, None) => {
                let w = if svg_size.width() > 0.0 {
                    svg_size.width().round() as u32
                } else {
                    self.default_width
                };
                
                let h = if svg_size.height() > 0.0 {
                    svg_size.height().round() as u32
                } else {
                    self.default_height
                };
                
                Ok((w, h))
            }
        }
    }
    
    fn pixmap_to_image_buffer(&self, pixmap: tiny_skia::Pixmap) -> Result<ImageData, JsValue> {
        let width = pixmap.width();
        let height = pixmap.height();
        let pixels = pixmap.data();
        
        // tiny-skia uses RGBA premultiplied format, need to convert to regular RGBA
        let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
        
        for chunk in pixels.chunks_exact(4) {
            let [b, g, r, a] = [chunk[0], chunk[1], chunk[2], chunk[3]];
            
            // Convert from premultiplied BGRA to regular RGBA
            if a == 0 {
                rgba_data.extend_from_slice(&[0, 0, 0, 0]);
            } else {
                let alpha_f = a as f32 / 255.0;
                let r_unpremul = (r as f32 / alpha_f).round().min(255.0) as u8;
                let g_unpremul = (g as f32 / alpha_f).round().min(255.0) as u8;
                let b_unpremul = (b as f32 / alpha_f).round().min(255.0) as u8;
                
                rgba_data.extend_from_slice(&[r_unpremul, g_unpremul, b_unpremul, a]);
            }
        }
        
        ImageBuffer::from_raw(width, height, rgba_data)
            .ok_or_else(|| JsValue::from_str("Failed to create image buffer from rendered SVG"))
    }
}

// Convenience functions for common use cases
pub fn render_svg_with_size(
    svg_bytes: &[u8], 
    width: u32, 
    height: u32
) -> Result<ImageData, JsValue> {
    let renderer = SvgRenderer::new();
    let options = SvgRenderOptions {
        width: Some(width),
        height: Some(height),
        ..Default::default()
    };
    renderer.render_svg_to_image(svg_bytes, options)
}

pub fn render_svg_auto_size(svg_bytes: &[u8]) -> Result<ImageData, JsValue> {
    let renderer = SvgRenderer::new();
    let options = SvgRenderOptions::default();
    renderer.render_svg_to_image(svg_bytes, options)
}

pub fn render_svg_with_background(
    svg_bytes: &[u8], 
    width: Option<u32>, 
    height: Option<u32>,
    background: [u8; 4]
) -> Result<ImageData, JsValue> {
    let renderer = SvgRenderer::new();
    let options = SvgRenderOptions {
        width,
        height,
        background_color: Some(background),
        ..Default::default()
    };
    renderer.render_svg_to_image(svg_bytes, options)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_svg() -> &'static [u8] {
        b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\" viewBox=\"0 0 100 100\">
            <rect x=\"10\" y=\"10\" width=\"80\" height=\"80\" fill=\"red\" />
          </svg>"
    }
    
    #[test]
    fn test_render_svg_with_explicit_size() {
        let svg_data = create_test_svg();
        let result = render_svg_with_size(svg_data, 200, 200);
        assert!(result.is_ok());
        
        let image = result.unwrap();
        assert_eq!(image.width(), 200);
        assert_eq!(image.height(), 200);
    }
    
    #[test]
    fn test_render_svg_auto_size() {
        let svg_data = create_test_svg();
        let result = render_svg_auto_size(svg_data);
        assert!(result.is_ok());
        
        let image = result.unwrap();
        // Should use the SVG's intrinsic size
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 100);
    }
}
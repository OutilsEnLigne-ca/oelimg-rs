use crate::filters::{ImageFilter, ImageData};
use crate::core::color_space::{Rgb, Hsl};
use crate::utils::clamp_f32;
use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SepiaFilter {
    intensity: f32, // 0.0 to 1.0
}

#[wasm_bindgen]
impl SepiaFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(intensity: f32) -> Self {
        Self {
            intensity: clamp_f32(intensity, 0.0, 1.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    #[wasm_bindgen(setter)]
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = clamp_f32(intensity, 0.0, 1.0);
    }

    pub fn full() -> Self {
        Self::new(1.0)
    }

    pub fn subtle() -> Self {
        Self::new(0.5)
    }
}

impl ImageFilter for SepiaFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        if self.intensity <= 0.01 {
            return Ok(()); // No sepia effect needed
        }

        for pixel in image.pixels_mut() {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            
            // Standard sepia transformation matrix
            let sepia_r = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0);
            let sepia_g = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0);
            let sepia_b = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0);
            
            // Blend with original based on intensity
            pixel[0] = (r * (1.0 - self.intensity) + sepia_r * self.intensity) as u8;
            pixel[1] = (g * (1.0 - self.intensity) + sepia_g * self.intensity) as u8;
            pixel[2] = (b * (1.0 - self.intensity) + sepia_b * self.intensity) as u8;
            // Alpha remains unchanged
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Sepia"
    }
}

#[wasm_bindgen]
pub struct VintageFilter {
    warmth: f32,        // 0.0 to 2.0 (1.0 = neutral)
    vignette: f32,      // 0.0 to 1.0
    grain: f32,         // 0.0 to 1.0
    fade: f32,          // 0.0 to 1.0 (film fade effect)
}

#[wasm_bindgen]
impl VintageFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(warmth: f32, vignette: f32, grain: f32, fade: f32) -> Self {
        Self {
            warmth: clamp_f32(warmth, 0.0, 2.0),
            vignette: clamp_f32(vignette, 0.0, 1.0),
            grain: clamp_f32(grain, 0.0, 1.0),
            fade: clamp_f32(fade, 0.0, 1.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn warmth(&self) -> f32 {
        self.warmth
    }

    #[wasm_bindgen(getter)]
    pub fn vignette(&self) -> f32 {
        self.vignette
    }

    #[wasm_bindgen(getter)]
    pub fn grain(&self) -> f32 {
        self.grain
    }

    #[wasm_bindgen(getter)]
    pub fn fade(&self) -> f32 {
        self.fade
    }

    // Preset vintage styles
    pub fn classic() -> Self {
        Self::new(1.3, 0.4, 0.2, 0.1)
    }

    pub fn warm() -> Self {
        Self::new(1.6, 0.3, 0.15, 0.15)
    }

    pub fn faded() -> Self {
        Self::new(1.1, 0.5, 0.1, 0.4)
    }
}

impl ImageFilter for VintageFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        let width = image.width();
        let height = image.height();
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let max_distance = ((width * width + height * height) as f32).sqrt() / 2.0;
        
        // Simple pseudo-random for grain effect
        let mut noise_seed = 12345u32;
        
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel_mut(x, y);
                let rgb = Rgb::from_u8(pixel[0], pixel[1], pixel[2]);
                
                // Apply warmth (color temperature shift)
                let warm_rgb = if self.warmth != 1.0 {
                    let factor = (self.warmth - 1.0) * 0.3;
                    Rgb::new(
                        clamp_f32(rgb.r + factor * 0.5, 0.0, 1.0),
                        clamp_f32(rgb.g + factor * 0.2, 0.0, 1.0),
                        clamp_f32(rgb.b - factor * 0.1, 0.0, 1.0),
                    )
                } else {
                    rgb
                };
                
                // Apply vignette effect
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                let vignette_factor = 1.0 - (distance / max_distance * self.vignette).min(1.0);
                
                let vignette_rgb = Rgb::new(
                    warm_rgb.r * vignette_factor,
                    warm_rgb.g * vignette_factor,
                    warm_rgb.b * vignette_factor,
                );
                
                // Apply film fade (lift shadows, reduce contrast)
                let fade_rgb = if self.fade > 0.01 {
                    let lift_amount = self.fade * 0.1;
                    let contrast_reduction = 1.0 - self.fade * 0.2;
                    
                    Rgb::new(
                        clamp_f32((vignette_rgb.r - 0.5) * contrast_reduction + 0.5 + lift_amount, 0.0, 1.0),
                        clamp_f32((vignette_rgb.g - 0.5) * contrast_reduction + 0.5 + lift_amount, 0.0, 1.0),
                        clamp_f32((vignette_rgb.b - 0.5) * contrast_reduction + 0.5 + lift_amount, 0.0, 1.0),
                    )
                } else {
                    vignette_rgb
                };
                
                // Apply grain
                let final_rgb = if self.grain > 0.01 {
                    // Simple pseudo-random noise
                    noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
                    let noise = ((noise_seed >> 16) & 0xFF) as f32 / 255.0 - 0.5;
                    let grain_amount = self.grain * 0.05 * noise;
                    
                    Rgb::new(
                        clamp_f32(fade_rgb.r + grain_amount, 0.0, 1.0),
                        clamp_f32(fade_rgb.g + grain_amount, 0.0, 1.0),
                        clamp_f32(fade_rgb.b + grain_amount, 0.0, 1.0),
                    )
                } else {
                    fade_rgb
                };
                
                let (r, g, b) = final_rgb.to_u8();
                pixel[0] = r;
                pixel[1] = g;
                pixel[2] = b;
                // Alpha remains unchanged
            }
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Vintage"
    }
}

#[wasm_bindgen]
pub struct FilmGrainFilter {
    intensity: f32,    // 0.0 to 1.0
    size: f32,         // 0.5 to 5.0 (grain particle size)
    roughness: f32,    // 0.0 to 1.0 (grain roughness/sharpness)
}

#[wasm_bindgen]
impl FilmGrainFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(intensity: f32, size: f32, roughness: f32) -> Self {
        Self {
            intensity: clamp_f32(intensity, 0.0, 1.0),
            size: clamp_f32(size, 0.5, 5.0),
            roughness: clamp_f32(roughness, 0.0, 1.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> f32 {
        self.size
    }

    #[wasm_bindgen(getter)]
    pub fn roughness(&self) -> f32 {
        self.roughness
    }

    // Preset grain types
    pub fn fine() -> Self {
        Self::new(0.3, 1.0, 0.7)
    }

    pub fn medium() -> Self {
        Self::new(0.5, 2.0, 0.5)
    }

    pub fn coarse() -> Self {
        Self::new(0.7, 3.5, 0.3)
    }
}

impl ImageFilter for FilmGrainFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        if self.intensity <= 0.01 {
            return Ok(()); // No grain effect needed
        }

        let width = image.width();
        let height = image.height();
        
        // Create noise pattern
        let mut noise_seed = 42u32;
        
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel_mut(x, y);
                
                // Generate noise based on position and size
                let noise_x = (x as f32 / self.size) as u32;
                let noise_y = (y as f32 / self.size) as u32;
                
                // Simple hash for consistent noise pattern
                let hash_input = noise_x.wrapping_mul(73856093) ^ noise_y.wrapping_mul(19349663);
                noise_seed = hash_input.wrapping_mul(1103515245).wrapping_add(12345);
                
                let raw_noise = ((noise_seed >> 16) & 0xFF) as f32 / 255.0;
                
                // Apply roughness (sharp vs smooth grain)
                let noise = if self.roughness > 0.5 {
                    // Sharp grain - more binary
                    (if raw_noise > 0.5 { 1.0 } else { -1.0 }) * self.roughness
                } else {
                    // Smooth grain - continuous
                    (raw_noise - 0.5) * 2.0 * (1.0 - self.roughness)
                };
                
                // Apply grain to each color channel
                for c in 0..3 {
                    let original = pixel[c] as f32 / 255.0;
                    let grain_amount = noise * self.intensity * 0.1;
                    
                    // Grain affects shadows and highlights differently
                    let luminance_factor = 1.0 - (original - 0.5).abs() * 2.0; // More grain in midtones
                    let final_grain = grain_amount * luminance_factor;
                    
                    let new_value = original + final_grain;
                    pixel[c] = (new_value.clamp(0.0, 1.0) * 255.0) as u8;
                }
                // Alpha remains unchanged
            }
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Film Grain"
    }
}

#[wasm_bindgen]
pub struct CrossProcessFilter {
    intensity: f32, // 0.0 to 1.0
    style: u8,      // 0-3 for different cross-processing styles
}

#[wasm_bindgen]
impl CrossProcessFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(intensity: f32, style: u8) -> Self {
        Self {
            intensity: clamp_f32(intensity, 0.0, 1.0),
            style: style.min(3),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    #[wasm_bindgen(getter)]
    pub fn style(&self) -> u8 {
        self.style
    }

    // Preset cross-processing styles
    pub fn style_1() -> Self {
        Self::new(0.8, 0)
    }

    pub fn style_2() -> Self {
        Self::new(0.8, 1)
    }

    pub fn style_3() -> Self {
        Self::new(0.8, 2)
    }
}

impl ImageFilter for CrossProcessFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        if self.intensity <= 0.01 {
            return Ok(());
        }

        for pixel in image.pixels_mut() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            
            // Different cross-processing curve styles
            let (new_r, new_g, new_b) = match self.style {
                0 => {
                    // Style 1: Enhanced contrast with color shifts
                    let r_curve = curve_enhance_contrast(r, 1.2);
                    let g_curve = curve_s_curve(g, 0.8);
                    let b_curve = curve_lift_gamma(b, 0.1, 1.1);
                    (r_curve, g_curve, b_curve)
                },
                1 => {
                    // Style 2: Vintage with blue-yellow shift
                    let r_curve = curve_s_curve(r, 1.1);
                    let g_curve = curve_lift_gamma(g, -0.05, 0.9);
                    let b_curve = curve_enhance_contrast(b, 0.8);
                    (r_curve, g_curve, b_curve)
                },
                2 => {
                    // Style 3: High contrast with lifted shadows
                    let r_curve = curve_lift_gamma(r, 0.1, 1.2);
                    let g_curve = curve_enhance_contrast(g, 1.1);
                    let b_curve = curve_s_curve(b, 0.9);
                    (r_curve, g_curve, b_curve)
                },
                _ => {
                    // Style 4: Faded look
                    let r_curve = curve_lift_gamma(r, 0.15, 0.85);
                    let g_curve = curve_lift_gamma(g, 0.1, 0.9);
                    let b_curve = curve_lift_gamma(b, 0.05, 0.95);
                    (r_curve, g_curve, b_curve)
                }
            };
            
            // Blend with original based on intensity
            pixel[0] = ((r * (1.0 - self.intensity) + new_r * self.intensity).clamp(0.0, 1.0) * 255.0) as u8;
            pixel[1] = ((g * (1.0 - self.intensity) + new_g * self.intensity).clamp(0.0, 1.0) * 255.0) as u8;
            pixel[2] = ((b * (1.0 - self.intensity) + new_b * self.intensity).clamp(0.0, 1.0) * 255.0) as u8;
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Cross Process"
    }
}

// Helper functions for curve adjustments
fn curve_s_curve(value: f32, strength: f32) -> f32 {
    // S-curve for enhanced contrast
    let x = value.clamp(0.0, 1.0);
    let adjusted = ((x - 0.5) * strength + 0.5).clamp(0.0, 1.0);
    if adjusted < 0.5 {
        2.0 * adjusted * adjusted
    } else {
        1.0 - 2.0 * (1.0 - adjusted) * (1.0 - adjusted)
    }
}

fn curve_enhance_contrast(value: f32, factor: f32) -> f32 {
    ((value - 0.5) * factor + 0.5).clamp(0.0, 1.0)
}

fn curve_lift_gamma(value: f32, lift: f32, gamma: f32) -> f32 {
    let lifted = (value + lift).clamp(0.0, 1.0);
    lifted.powf(1.0 / gamma)
}
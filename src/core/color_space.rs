use crate::utils::clamp_f32;

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    pub h: f32, // 0.0 - 360.0
    pub s: f32, // 0.0 - 1.0
    pub l: f32, // 0.0 - 1.0
}

#[derive(Debug, Clone, Copy)]
pub struct Lab {
    pub l: f32, // 0.0 - 100.0
    pub a: f32, // -128.0 - 127.0
    pub b: f32, // -128.0 - 127.0
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: clamp_f32(r, 0.0, 1.0),
            g: clamp_f32(g, 0.0, 1.0),
            b: clamp_f32(b, 0.0, 1.0),
        }
    }

    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }

    pub fn to_hsl(&self) -> Hsl {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let lightness = (max + min) / 2.0;

        if delta == 0.0 {
            return Hsl {
                h: 0.0,
                s: 0.0,
                l: lightness,
            };
        }

        let saturation = if lightness < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        let hue = if max == self.r {
            (self.g - self.b) / delta + (if self.g < self.b { 6.0 } else { 0.0 })
        } else if max == self.g {
            (self.b - self.r) / delta + 2.0
        } else {
            (self.r - self.g) / delta + 4.0
        } * 60.0;

        Hsl {
            h: hue,
            s: saturation,
            l: lightness,
        }
    }

    // Adjust brightness while preserving color relationships
    pub fn adjust_brightness(&self, factor: f32) -> Self {
        Self {
            r: clamp_f32(self.r * factor, 0.0, 1.0),
            g: clamp_f32(self.g * factor, 0.0, 1.0),
            b: clamp_f32(self.b * factor, 0.0, 1.0),
        }
    }

    // Adjust contrast around midpoint (0.5)
    pub fn adjust_contrast(&self, factor: f32) -> Self {
        Self {
            r: clamp_f32((self.r - 0.5) * factor + 0.5, 0.0, 1.0),
            g: clamp_f32((self.g - 0.5) * factor + 0.5, 0.0, 1.0),
            b: clamp_f32((self.b - 0.5) * factor + 0.5, 0.0, 1.0),
        }
    }
}

impl Hsl {
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self {
            h: h % 360.0,
            s: clamp_f32(s, 0.0, 1.0),
            l: clamp_f32(l, 0.0, 1.0),
        }
    }

    pub fn to_rgb(&self) -> Rgb {
        if self.s == 0.0 {
            return Rgb::new(self.l, self.l, self.l);
        }

        let hue_to_rgb = |p: f32, q: f32, t: f32| -> f32 {
            let mut t = t;
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * t;
            }
            if t < 1.0 / 2.0 {
                return q;
            }
            if t < 2.0 / 3.0 {
                return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
            }
            p
        };

        let q = if self.l < 0.5 {
            self.l * (1.0 + self.s)
        } else {
            self.l + self.s - self.l * self.s
        };
        
        let p = 2.0 * self.l - q;
        let h_norm = self.h / 360.0;

        Rgb {
            r: hue_to_rgb(p, q, h_norm + 1.0 / 3.0),
            g: hue_to_rgb(p, q, h_norm),
            b: hue_to_rgb(p, q, h_norm - 1.0 / 3.0),
        }
    }

    // Adjust hue by degrees
    pub fn adjust_hue(&self, degrees: f32) -> Self {
        Self {
            h: (self.h + degrees) % 360.0,
            s: self.s,
            l: self.l,
        }
    }

    // Adjust saturation
    pub fn adjust_saturation(&self, factor: f32) -> Self {
        Self {
            h: self.h,
            s: clamp_f32(self.s * factor, 0.0, 1.0),
            l: self.l,
        }
    }

    // Adjust lightness
    pub fn adjust_lightness(&self, factor: f32) -> Self {
        Self {
            h: self.h,
            s: self.s,
            l: clamp_f32(self.l * factor, 0.0, 1.0),
        }
    }
}

// Color temperature adjustment functions
pub fn adjust_color_temperature(rgb: Rgb, kelvin: f32) -> Rgb {
    // Simplified color temperature adjustment
    // Based on approximation for daylight temperatures 1000K-40000K
    let temp = clamp_f32(kelvin, 1000.0, 40000.0) / 100.0;
    
    let (temp_r, temp_g, temp_b) = if temp <= 66.0 {
        let r = 255.0;
        let g = temp;
        let g = 99.4708025861 * (g).ln() - 161.1195681661;
        let b = if temp >= 19.0 {
            let b = temp - 10.0;
            138.5177312231 * (b).ln() - 305.0447927307
        } else {
            0.0
        };
        (r, g, b)
    } else {
        let r = temp - 60.0;
        let r = 329.698727446 * (r).powf(-0.1332047592);
        let g = temp - 60.0;
        let g = 288.1221695283 * (g).powf(-0.0755148492);
        let b = 255.0;
        (r, g, b)
    };

    let temp_r = clamp_f32(temp_r / 255.0, 0.0, 1.0);
    let temp_g = clamp_f32(temp_g / 255.0, 0.0, 1.0);
    let temp_b = clamp_f32(temp_b / 255.0, 0.0, 1.0);

    // Blend with original color
    let strength = 0.3; // Adjust intensity of temperature effect
    Rgb {
        r: rgb.r * (1.0 - strength) + temp_r * strength,
        g: rgb.g * (1.0 - strength) + temp_g * strength,
        b: rgb.b * (1.0 - strength) + temp_b * strength,
    }
}
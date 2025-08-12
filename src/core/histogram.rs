use crate::filters::ImageData;

#[derive(Debug, Clone)]
pub struct Histogram {
    pub red: Vec<u32>,
    pub green: Vec<u32>,
    pub blue: Vec<u32>,
    pub luminance: Vec<u32>,
}

impl Histogram {
    pub fn new() -> Self {
        Self {
            red: vec![0; 256],
            green: vec![0; 256],
            blue: vec![0; 256],
            luminance: vec![0; 256],
        }
    }

    pub fn from_image(image: &ImageData) -> Self {
        let mut histogram = Self::new();
        
        for pixel in image.pixels() {
            let r = pixel[0] as usize;
            let g = pixel[1] as usize;
            let b = pixel[2] as usize;
            
            histogram.red[r] += 1;
            histogram.green[g] += 1;
            histogram.blue[b] += 1;
            
            // Calculate luminance using ITU-R BT.709 weights
            let luminance = (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as usize;
            histogram.luminance[luminance.min(255)] += 1;
        }
        
        histogram
    }

    pub fn get_percentile(&self, channel: HistogramChannel, percentile: f32) -> u8 {
        let data = match channel {
            HistogramChannel::Red => &self.red,
            HistogramChannel::Green => &self.green,
            HistogramChannel::Blue => &self.blue,
            HistogramChannel::Luminance => &self.luminance,
        };

        let total_pixels: u32 = data.iter().sum();
        let target_pixels = (total_pixels as f32 * percentile / 100.0) as u32;
        
        let mut cumulative = 0;
        for (i, &count) in data.iter().enumerate() {
            cumulative += count;
            if cumulative >= target_pixels {
                return i as u8;
            }
        }
        
        255
    }

    pub fn get_mean(&self, channel: HistogramChannel) -> f32 {
        let data = match channel {
            HistogramChannel::Red => &self.red,
            HistogramChannel::Green => &self.green,
            HistogramChannel::Blue => &self.blue,
            HistogramChannel::Luminance => &self.luminance,
        };

        let total_pixels: u32 = data.iter().sum();
        if total_pixels == 0 {
            return 0.0;
        }

        let weighted_sum: u32 = data
            .iter()
            .enumerate()
            .map(|(value, &count)| value as u32 * count)
            .sum();

        weighted_sum as f32 / total_pixels as f32
    }

    // Auto levels adjustment based on histogram
    pub fn calculate_auto_levels(&self) -> (f32, f32, f32) {
        let shadow_percentile = 1.0; // Remove darkest 1%
        let highlight_percentile = 99.0; // Remove brightest 1%
        
        let shadow_point = self.get_percentile(HistogramChannel::Luminance, shadow_percentile) as f32 / 255.0;
        let highlight_point = self.get_percentile(HistogramChannel::Luminance, highlight_percentile) as f32 / 255.0;
        
        // Calculate gamma for midtone adjustment
        let midtone = self.get_percentile(HistogramChannel::Luminance, 50.0) as f32 / 255.0;
        let gamma = if highlight_point > shadow_point && midtone > shadow_point {
            ((midtone - shadow_point) / (highlight_point - shadow_point)).ln() / 0.5_f32.ln()
        } else {
            1.0
        };

        (shadow_point, gamma, highlight_point)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HistogramChannel {
    Red,
    Green,
    Blue,
    Luminance,
}
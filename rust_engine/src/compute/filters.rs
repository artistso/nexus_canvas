use nalgebra::Vector2;
use std::sync::Arc;

// These structs are passed to the WGSL shaders.
#[repr(C)]
pub struct FilterParams {
    pub brightness: f32, // -1.0 to 1.0
    pub contrast: f32,   // 0.0 to 3.0
    pub saturation: f32, // 0.0 to 2.0
    pub hue_shift: f32,  // -180.0 to 180.0
    pub blur_radius: f32,
    pub width: u32,
    pub height: u32,
}

pub struct GpuFilter {
    pub params: FilterParams,
    // In production, we'd hold wgpu bind groups here.
}

impl GpuFilter {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            params: FilterParams {
                brightness: 0.0,
                contrast: 1.0,
                saturation: 1.0,
                hue_shift: 0.0,
                blur_radius: 0.0,
                width,
                height,
            },
        }
    }

    pub fn apply_brightness_contrast(&mut self, brightness: f32, contrast: f32) {
        self.params.brightness = brightness.clamp(-1.0, 1.0);
        self.params.contrast = contrast.clamp(0.0, 3.0);
        // Dispatch compute shader...
    }

    pub fn apply_hsl(&mut self, hue: f32, saturation: f32, lightness: f32) {
        self.params.hue_shift = hue;
        self.params.saturation = saturation;
        // Dispatch compute shader...
    }

    pub fn apply_blur(&mut self, radius: f32) {
        self.params.blur_radius = radius;
        // Dispatch compute shader (two-pass Gaussian)
    }

    // RGB -> HSL conversion for CPU fallback
    pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;
        if max == min {
            return (0.0, 0.0, l);
        }
        let d = max - min;
        let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
        let h = if max == r {
            (g - b) / d + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };
        (h / 6.0, s, l)
    }
}

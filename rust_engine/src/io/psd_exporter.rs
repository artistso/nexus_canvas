use crate::ecs::layer_stack::Layer;
use std::io::Write;

// Simplified PSD writer
pub struct PsdExporter {
    layers: Vec<Layer>,
    width: u32,
    height: u32,
}

impl PsdExporter {
    pub fn new(layers: Vec<Layer>, width: u32, height: u32) -> Self {
        Self { layers, width, height }
    }

    pub fn export(&self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;

        // PSD Header (Signature, Version, Channels, Height, Width, Depth, ColorMode)
        let header = b"8BPS\0\0\0\0\0\0\0\0";
        file.write_all(header)?;

        // Write dimensions (big endian)
        let height_bytes = (self.height as u32).to_be_bytes();
        let width_bytes = (self.width as u32).to_be_bytes();
        file.write_all(&height_bytes)?;
        file.write_all(&width_bytes)?;

        // Depth (8 bits per channel)
        file.write_all(&[0x08])?;

        // Color Mode: RGB (0x03)
        file.write_all(&[0x03])?;

        // Image Resources (empty for now)
        let res_len: u32 = 0;
        file.write_all(&res_len.to_be_bytes())?;

        // Layer & Mask Info Section (simplified)
        // We build layer records here
        let mut layer_data = Vec::new();
        for layer in &self.layers {
            // Layer record: Top, Left, Bottom, Right (4 bytes each)
            let top = 0_i32.to_be_bytes();
            let left = 0_i32.to_be_bytes();
            let bottom = (self.height as i32).to_be_bytes();
            let right = (self.width as i32).to_be_bytes();
            layer_data.extend_from_slice(&top);
            layer_data.extend_from_slice(&left);
            layer_data.extend_from_slice(&bottom);
            layer_data.extend_from_slice(&right);

            // Channel count (4: RGBA)
            layer_data.extend_from_slice(&[0x00, 0x04]);

            // Channels header (length 4 bytes per channel)
            // For simplicity, we just write the minimum.

            // Blend mode signature
            layer_data.extend_from_slice(b"8BIM");
            // Blend mode key (e.g., "norm")
            let blend_mode = match layer.blend_mode {
                crate::ecs::layer_stack::BlendMode::Normal => b"norm",
                crate::ecs::layer_stack::BlendMode::Multiply => b"mul ",
                crate::ecs::layer_stack::BlendMode::Screen => b"scrn",
                _ => b"norm",
            };
            layer_data.extend_from_slice(blend_mode);

            // Opacity and visibility
            let opacity = (layer.opacity * 255.0) as u8;
            layer_data.push(opacity);
            layer_data.push(0); // Clipping
            layer_data.push(0); // Flags (visibility)
            layer_data.push(0); // Filler
        }

        // Write the layer data length
        let len = layer_data.len() as u32;
        file.write_all(&len.to_be_bytes())?;
        file.write_all(&layer_data)?;

        // Write pixel data (stub - in production we export actual images)
        file.sync_all()?;
        Ok(())
    }
}

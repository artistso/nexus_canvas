// This module interfaces with the GPU via wgpu (via Skia's backend or direct wgpu).
// For now, we define the structs that will be passed to the shader.

#[repr(C)]
pub struct DisplacementGrid {
    pub width: u32,
    pub height: u32,
    pub displacements: Vec<[f32; 2]>, // x, y displacement per vertex
}

impl DisplacementGrid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            displacements: vec![[0.0, 0.0]; (width * height) as usize],
        }
    }

    pub fn apply_force(&mut self, x: u32, y: u32, strength: f32, radius: f32) {
        let center_x = x as f32;
        let center_y = y as f32;
        for (idx, disp) in self.displacements.iter_mut().enumerate() {
            let gx = (idx % self.width as usize) as f32;
            let gy = (idx / self.width as usize) as f32;
            let dx = gx - center_x;
            let dy = gy - center_y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < radius {
                let falloff = (1.0 - (dist / radius)) * strength;
                disp[0] += dx.signum() * falloff * 0.5;
                disp[1] += dy.signum() * falloff * 0.5;
            }
        }
    }
}

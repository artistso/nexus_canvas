use serde::{Serialize, Deserialize};
use bevy_ecs::prelude::*;
use nalgebra::{Matrix3, Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub center: Vector2<f32>,   // World center of the viewport
    pub scale: f32,             // Zoom level (1.0 = 100%)
    pub rotation: f32,          // Rotation in radians
    pub screen_width: f32,
    pub screen_height: f32,
}

impl Viewport {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            center: Vector2::new(0.0, 0.0),
            scale: 1.0,
            rotation: 0.0,
            screen_width,
            screen_height,
        }
    }

    // Convert screen coordinates (pixels) to world coordinates
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> Vector2<f32> {
        let half_w = self.screen_width / 2.0;
        let half_h = self.screen_height / 2.0;

        // Normalize to -1..1
        let mut x = (screen_x - half_w) / half_w;
        let mut y = (screen_y - half_h) / half_h;

        // Apply inverse scale
        x /= self.scale;
        y /= self.scale;

        // Apply inverse rotation
        let cos = (-self.rotation).cos();
        let sin = (-self.rotation).sin();
        let rx = x * cos - y * sin;
        let ry = x * sin + y * cos;

        Vector2::new(rx + self.center.x, ry + self.center.y)
    }

    // Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_x: f32, world_y: f32) -> Vector2<f32> {
        let dx = world_x - self.center.x;
        let dy = world_y - self.center.y;

        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        let rx = dx * cos - dy * sin;
        let ry = dx * sin + dy * cos;

        let half_w = self.screen_width / 2.0;
        let half_h = self.screen_height / 2.0;

        Vector2::new(
            (rx * self.scale) * half_w + half_w,
            (ry * self.scale) * half_h + half_h,
        )
    }

    // Get the transformation matrix for Skia
    pub fn get_transform_matrix(&self) -> Matrix3<f32> {
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();

        // Scale + Rotate + Translate
        Matrix3::new(
            self.scale * cos, -self.scale * sin, self.center.x,
            self.scale * sin,  self.scale * cos, self.center.y,
            0.0,              0.0,              1.0,
        )
    }

    pub fn zoom(&mut self, factor: f32, anchor_x: f32, anchor_y: f32) {
        // Zoom towards the anchor point (usually finger position)
        let anchor_world = self.screen_to_world(anchor_x, anchor_y);
        let new_scale = (self.scale * factor).clamp(0.01, 100.0);
        let scale_ratio = new_scale / self.scale;
        self.center = anchor_world + (self.center - anchor_world) * scale_ratio;
        self.scale = new_scale;
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        // dx, dy are in screen pixels. Convert to world units.
        let world_dx = dx / self.scale;
        let world_dy = dy / self.scale;
        self.center.x -= world_dx;
        self.center.y -= world_dy;
    }
}

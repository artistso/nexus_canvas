use bevy_ecs::prelude::*;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct VectorPath {
    pub control_points: Vec<Vector2<f32>>, // Flat list: [p0, p1, p2, p3, ...]
    pub closed: bool,
}

impl VectorPath {
    pub fn new() -> Self {
        Self {
            control_points: Vec::new(),
            closed: false,
        }
    }

    // Catmull-Rom to Bezier conversion is handled in the renderer.
    // Here we just store the raw smoothed points.
    pub fn add_point(&mut self, point: Vector2<f32>) {
        self.control_points.push(point);
    }

    pub fn get_cubic_segments(&self) -> Vec<[Vector2<f32>; 4]> {
        let mut segments = Vec::new();
        if self.control_points.len() < 4 {
            return segments;
        }
        // Simple sliding window of 4 points -> cubic bezier
        for i in 0..(self.control_points.len() - 3) {
            let p0 = self.control_points[i];
            let p1 = self.control_points[i + 1];
            let p2 = self.control_points[i + 2];
            let p3 = self.control_points[i + 3];
            segments.push([p0, p1, p2, p3]);
        }
        segments
    }
}

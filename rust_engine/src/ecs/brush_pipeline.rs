use bevy_ecs::prelude::*;
use nalgebra::Vector2;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrushConfig {
    pub name: String,
    pub size: f32,
    pub opacity: f32,
    pub spacing: f32,            // Distance between dots (0.01 to 1.0)
    pub scatter: f32,            // Random offset from path
    pub rotation: f32,           // Brush tip rotation in radians
    pub jitter_size: f32,
    pub jitter_opacity: f32,
    pub pressure_sensitivity: f32,
    pub tilt_sensitivity: f32,
    pub wetness: f32,            // 0.0 (dry) to 1.0 (watercolor bleed)
    pub flow: f32,               // How much paint is deposited per step
}

impl Default for BrushConfig {
    fn default() -> Self {
        Self {
            name: "Round Brush".to_string(),
            size: 10.0,
            opacity: 1.0,
            spacing: 0.05,
            scatter: 0.0,
            rotation: 0.0,
            jitter_size: 0.0,
            jitter_opacity: 0.0,
            pressure_sensitivity: 1.0,
            tilt_sensitivity: 0.5,
            wetness: 0.0,
            flow: 1.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct BrushStroke {
    pub config: BrushConfig,
    pub points: Vec<Point>,
    pub current_position: Vector2<f32>,
    pub distance_since_last_dot: f32,
}

impl BrushStroke {
    pub fn new(config: BrushConfig, start_x: f32, start_y: f32) -> Self {
        Self {
            config,
            points: vec![Point { x: start_x, y: start_y, pressure: 0.5, tilt: 0.0, azimuth: 0.0 }],
            current_position: Vector2::new(start_x, start_y),
            distance_since_last_dot: 0.0,
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32, pressure: f32, tilt: f32, azimuth: f32) {
        let new_point = Vector2::new(x, y);
        let dist = (new_point - self.current_position).magnitude();
        self.current_position = new_point;
        self.distance_since_last_dot += dist;

        // Spacing logic: place dots based on spacing parameter
        let spacing_px = self.config.spacing * self.config.size * 2.0;
        while self.distance_since_last_dot >= spacing_px {
            self.distance_since_last_dot -= spacing_px;
            let t = 1.0 - (self.distance_since_last_dot / dist);
            // Interpolate between last point and new point
            // Store the dot
            let dot = Point {
                x,
                y,
                pressure: pressure * self.config.pressure_sensitivity,
                tilt: tilt * self.config.tilt_sensitivity,
                azimuth,
            };
            self.points.push(dot);
        }
    }

    pub fn generate_dots_with_jitter(&self) -> Vec<Point> {
        let mut rng = rand::thread_rng();
        let mut result = Vec::new();

        for point in &self.points {
            let size_jitter = (rng.gen::<f32>() - 0.5) * self.config.jitter_size * self.config.size;
            let opacity_jitter = (rng.gen::<f32>() - 0.5) * self.config.jitter_opacity;
            let scatter_x = (rng.gen::<f32>() - 0.5) * self.config.scatter * self.config.size;
            let scatter_y = (rng.gen::<f32>() - 0.5) * self.config.scatter * self.config.size;

            result.push(Point {
                x: point.x + scatter_x,
                y: point.y + scatter_y,
                pressure: (point.pressure + opacity_jitter).clamp(0.0, 1.0),
                tilt: point.tilt,
                azimuth: point.azimuth,
            });
        }
        result
    }
}

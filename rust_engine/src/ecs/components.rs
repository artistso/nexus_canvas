use bevy_ecs::prelude::*;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
    pub timestamp: u64,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub id: u32,
    pub points: Vec<Point>,
    pub color_hex: String,
    pub width: f32,
    pub layer_id: u32,
    pub is_finished: bool,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: u32,
    pub name: String,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    HardLight,
    SoftLight,
    ColorDodge,
    ColorBurn,
}

#[derive(Component, Debug, Clone)]
pub struct TrailQueue {
    pub points: Vec<Point>,
    pub max_length: u32,
}

impl TrailQueue {
    pub fn new(max_length: u32) -> Self {
        Self {
            points: Vec::with_capacity(max_length as usize),
            max_length,
        }
    }

    pub fn push(&mut self, point: Point) {
        self.points.push(point);
        if self.points.len() > self.max_length as usize {
            self.points.remove(0);
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Theme {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub shades: Vec<String>,
}

#[derive(Resource, Debug, Default)]
pub struct UndoStack {
    pub operations: Vec<String>, // serialized events
    pub index: usize,
}

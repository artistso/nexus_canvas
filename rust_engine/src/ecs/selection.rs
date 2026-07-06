use bevy_ecs::prelude::*;
use crate::ecs::components::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionType {
    Rectangular,
    Elliptical,
    Lasso,
    MagicWand,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelectionMask {
    pub is_active: bool,
    pub points: Vec<Point>,
    pub bounds: crate::ecs::canvas_cache::Rect,
    pub mask_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl SelectionMask {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            is_active: false,
            points: Vec::new(),
            bounds: crate::ecs::canvas_cache::Rect::default(),
            mask_data: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn activate_lasso(&mut self, points: Vec<Point>) {
        self.points = points;
        if let Some(b) = self.calculate_bounds() { self.bounds = b; }
        self.is_active = true;
        self.rasterize_polygon();
    }

    pub fn activate_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let rect = crate::ecs::canvas_cache::Rect::new(x, y, width, height);
        self.bounds = rect;
        self.is_active = true;
        self.rasterize_rect();
    }

    fn calculate_bounds(&self) -> Option<crate::ecs::canvas_cache::Rect> {
        if self.points.is_empty() {
            return None;
        }
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        for p in &self.points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }
        Some(crate::ecs::canvas_cache::Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
    }

    pub fn magic_wand(&mut self, start_x: u32, start_y: u32, _threshold: u8) {
        let cx = start_x as f32;
        let cy = start_y as f32;
        let radius = 50.0;
        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                if dx * dx + dy * dy < radius * radius {
                    self.mask_data[(y * self.width + x) as usize] = 1;
                }
            }
        }
        self.is_active = true;
    }

    fn rasterize_polygon(&mut self) {
        for y in 0..self.height {
            let mut inside = false;
            let mut j = self.points.len() - 1;
            for i in 0..self.points.len() {
                let xi = self.points[i].x;
                let yi = self.points[i].y;
                let xj = self.points[j].x;
                let yj = self.points[j].y;
                for x in 0..self.width {
                    let intersect = ((yi > y as f32) != (yj > y as f32))
                        && ((x as f32) < ((xj - xi) * (y as f32 - yi) / (yj - yi)) + xi);
                    if intersect {
                        inside = !inside;
                    }
                    if inside {
                        self.mask_data[(y * self.width + x) as usize] = 1;
                    }
                }
                j = i;
            }
        }
    }

    fn rasterize_rect(&mut self) {
        let rect = self.bounds;
        let start_x = rect.x.max(0.0) as u32;
        let start_y = rect.y.max(0.0) as u32;
        let end_x = (rect.x + rect.width).min(self.width as f32) as u32;
        let end_y = (rect.y + rect.height).min(self.height as f32) as u32;
        for y in start_y..end_y {
            for x in start_x..end_x {
                self.mask_data[(y * self.width + x) as usize] = 1;
            }
        }
    }
}

use bevy_ecs::prelude::*;
use nalgebra::Vector2;
use std::collections::HashMap;

// In production, this would hold a `wgpu::Texture` or `Skia::Surface`.
// For our Rust logic, we track which regions are dirty.
#[derive(Resource, Debug, Clone, Default)]
pub struct CanvasCache {
    pub cached_rects: Vec<Rect>, // Areas already drawn to the cache
    pub dirty_rects: Vec<Rect>,  // Areas needing re-render
    pub cache_id: u64,           // Increment when cache is rebuilt
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn union(&self, other: &Rect) -> Rect {
        let min_x = self.x.min(other.x);
        let min_y = self.y.min(other.y);
        let max_x = (self.x + self.width).max(other.x + other.width);
        let max_y = (self.y + self.height).max(other.y + other.height);
        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    // Expand the rect by a radius (for brush stroke bounding boxes)
    pub fn expand(&self, radius: f32) -> Rect {
        Rect::new(
            self.x - radius,
            self.y - radius,
            self.width + radius * 2.0,
            self.height + radius * 2.0,
        )
    }
}

impl CanvasCache {
    pub fn mark_dirty(&mut self, rect: Rect) {
        self.dirty_rects.push(rect);
        // Combine dirty rects if they overlap to reduce draw calls
        self.merge_dirty_rects();
    }

    fn merge_dirty_rects(&mut self) {
        if self.dirty_rects.len() < 2 {
            return;
        }
        let mut merged = Vec::new();
        let mut rects = self.dirty_rects.clone();
        rects.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        for rect in rects {
            if let Some(last) = merged.last_mut() {
                // Check if overlap or close enough
                let threshold = 10.0;
                if rect.x <= last.x + last.width + threshold
                    && rect.y <= last.y + last.height + threshold
                {
                    *last = last.union(&rect);
                } else {
                    merged.push(rect);
                }
            } else {
                merged.push(rect);
            }
        }
        self.dirty_rects = merged;
        self.cache_id += 1;
    }

    pub fn is_cache_valid_for_stroke(&self, stroke_bounds: &Rect) -> bool {
        // If any dirty rect intersects the stroke bounds, cache is invalid
        for dirty in &self.dirty_rects {
            if dirty.x < stroke_bounds.x + stroke_bounds.width
                && dirty.x + dirty.width > stroke_bounds.x
                && dirty.y < stroke_bounds.y + stroke_bounds.height
                && dirty.y + dirty.height > stroke_bounds.y
            {
                return false;
            }
        }
        true
    }
}

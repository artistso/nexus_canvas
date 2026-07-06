use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: u32,
    pub name: String,
    pub opacity: f32,              // 0.0 to 1.0
    pub blend_mode: BlendMode,
    pub is_visible: bool,
    pub is_locked: bool,
    pub is_vector: bool,           // True = vector data, False = raster bitmap
    pub z_index: i32,              // Render order (higher = on top)
    pub width: u32,
    pub height: u32,
}

impl Layer {
    pub fn new(id: u32, width: u32, height: u32) -> Self {
        Self {
            id,
            name: format!("Layer {}", id),
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            is_visible: true,
            is_locked: false,
            is_vector: true,
            z_index: id as i32,
            width,
            height,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct LayerStack {
    pub layers: Vec<Layer>,
    pub active_layer_id: u32,
    pub next_layer_id: u32,
}

impl LayerStack {
    pub fn add_layer(&mut self, width: u32, height: u32) -> u32 {
        let id = self.next_layer_id;
        self.next_layer_id += 1;
        let layer = Layer::new(id, width, height);
        self.layers.push(layer);
        self.active_layer_id = id;
        self.sort_layers();
        id
    }

    pub fn remove_layer(&mut self, id: u32) -> bool {
        if self.layers.len() <= 1 {
            return false; // Keep at least one layer
        }
        if let Some(pos) = self.layers.iter().position(|l| l.id == id) {
            self.layers.remove(pos);
            if self.active_layer_id == id {
                self.active_layer_id = self.layers.first().map(|l| l.id).unwrap_or(0);
            }
            self.sort_layers();
            return true;
        }
        false
    }

    pub fn get_layer_mut(&mut self, id: u32) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == id)
    }

    pub fn move_layer(&mut self, id: u32, new_z_index: i32) {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == id) {
            layer.z_index = new_z_index;
        }
        self.sort_layers();
    }

    pub fn sort_layers(&mut self) {
        self.layers.sort_by_key(|l| l.z_index);
        // Reassign z_index sequentially for consistency
        for (i, layer) in self.layers.iter_mut().enumerate() {
            layer.z_index = i as i32;
        }
    }

    pub fn get_active_layer(&self) -> Option<&Layer> {
        self.layers.iter().find(|l| l.id == self.active_layer_id)
    }

    pub fn get_active_layer_mut(&mut self) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == self.active_layer_id)
    }
}

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CanvasEvent {
    StrokePoint {
        stroke_id: u32,
        x: f32,
        y: f32,
        pressure: f32,
    },
    StrokeFinished {
        stroke_id: u32,
    },
    ThemeChanged {
        new_primary: String,
        new_palette: Vec<String>,
    },
    LayerCreated {
        layer_id: u32,
        name: String,
    },
    LayerDeleted {
        layer_id: u32,
    },
    TransformApply {
        layer_id: u32,
        matrix: [[f32; 3]; 3],
    },
    Undo {
        steps: u32,
    },
    Redo {
        steps: u32,
    },
}

pub struct EventLog {
    pub events: Vec<CanvasEvent>,
    pub current_index: usize, // Where we are in history (for undo/redo)
    max_capacity: usize,
}

impl EventLog {
    pub fn new(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
            current_index: 0,
            max_capacity: capacity,
        }
    }

    // Push an event. If we are not at the tip, truncate the future.
    pub fn push(&mut self, event: CanvasEvent) {
        if self.current_index < self.events.len() {
            self.events.truncate(self.current_index);
        }
        self.events.push(event);
        if self.events.len() > self.max_capacity {
            self.events.remove(0);
        }
        self.current_index = self.events.len();
    }

    // Get all events up to the current index (for replay)
    pub fn get_active_events(&self) -> &[CanvasEvent] {
        &self.events[0..self.current_index]
    }

    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current_index < self.events.len()
    }

    pub fn undo(&mut self) -> Option<&CanvasEvent> {
        if self.can_undo() {
            self.current_index -= 1;
            self.events.get(self.current_index)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<&CanvasEvent> {
        if self.can_redo() {
            let event = self.events.get(self.current_index);
            self.current_index += 1;
            event
        } else {
            None
        }
    }

    // Serialize to compact binary for saving
    pub fn save_to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self.events).unwrap_or_default()
    }

    pub fn load_from_bytes(data: &[u8]) -> Self {
        let events: Vec<CanvasEvent> = bincode::deserialize(data).unwrap_or_default();
        let len = events.len();
        Self {
            events,
            current_index: len,
            max_capacity: 100_000,
        }
    }
}

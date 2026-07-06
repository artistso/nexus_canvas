use serde::{Deserialize, Serialize};
use crate::ecs::event_log::CanvasEvent;
use crate::ecs::layer_stack::{Layer, BlendMode};
use crate::ecs::viewport::Viewport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub author: String,
    pub created_at: u64,      // Unix timestamp
    pub modified_at: u64,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub dpi: u32,             // 72, 300, etc.
    pub color_profile: String, // sRGB, P3, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub metadata: ProjectMetadata,
    pub viewport: Viewport,
    pub layers: Vec<Layer>,
    pub events: Vec<CanvasEvent>, // Full undo history
    pub thumbnail: Vec<u8>,       // PNG preview (compressed)
}

impl ProjectFile {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap_or_default()
    }

    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        bincode::deserialize(data).ok()
    }

    // Save to .nexus extension (custom binary format)
    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let bytes = self.to_bytes();
        std::fs::write(path, bytes)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes).ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Nexus file format",
        ))
    }
}

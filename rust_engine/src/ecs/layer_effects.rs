use bevy_ecs::prelude::*;
use crate::ecs::components::{Layer, Point};

#[derive(Component, Debug, Clone)]
pub struct AlphaLock {
    pub enabled: bool,
}

#[derive(Component, Debug, Clone)]
pub struct ClippingMask {
    pub mask_layer_id: u32, // The layer below that provides the shape
    pub enabled: bool,
}

// System to apply clipping mask during rendering.
// This runs in the render pipeline just before drawing strokes.
pub fn apply_clipping_mask_system(
    layers: Query<&Layer>,
    clip_masks: Query<&ClippingMask>,
    mut stroke_query: Query<&mut Stroke>,
) {
    // In production, this system checks if the active layer has a clipping mask.
    // If so, it restricts the stroke's visible pixels to the shape of the mask layer.
    // For scaffolding, we just print a debug.
    // Implementation: Use stencil buffer.
    // Draw mask layer to stencil -> draw strokes only where stencil is 1.
}

// System to enforce alpha lock.
// When a stroke is added, if the layer has AlphaLock, new pixels can only overwrite
// existing opaque pixels.
pub fn apply_alpha_lock_system(
    mut stroke_query: Query<(&mut Stroke, &Layer)>,
    lock_query: Query<&AlphaLock>,
) {
    // For each stroke, check if its layer has AlphaLock enabled.
    // If enabled, truncate stroke points to only overlap with existing alpha > 0.
    // This requires reading the pixel buffer of the layer.
    // (Scaffolding stub - full implementation requires pixel buffer access.)
}

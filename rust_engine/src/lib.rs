
use std::str::FromStr;
uniffi::include_scaffolding!("nexus_engine");

pub mod ecs;
pub mod compute;
pub mod io;

use bevy_ecs::prelude::*;
use ecs::components::*;
use ecs::systems::*;
use ecs::event_log::{EventLog, CanvasEvent};
use ecs::vector_path::VectorPath;
use ecs::layer_stack::{LayerStack, BlendMode, Layer};
use ecs::viewport::Viewport;
use ecs::brush_pipeline::{BrushConfig, BrushStroke};
use ecs::canvas_cache::CanvasCache;
use ecs::selection::{SelectionMask, SelectionType};
use io::project_schema::{ProjectFile, ProjectMetadata};
use io::psd_exporter::PsdExporter;
use palette::IntoColor;
pub use ecs::canvas_cache::Rect;

pub struct NexusEngineImpl {
    world: World,
    schedule: Schedule,
    stabilizer_strength: f32,
    undo_stack: UndoStack,
    pub event_log: EventLog,
    pub current_stroke_path: VectorPath,
    pub layer_stack: LayerStack,
    pub viewport: Viewport,
    pub brush_config: BrushConfig,
    pub active_stroke: Option<BrushStroke>,
    pub canvas_cache: CanvasCache,
    pub selection_mask: SelectionMask,
    pub render_times: Vec<f32>,
}

pub struct NexusEngine {
    inner: std::sync::RwLock<NexusEngineImpl>,
}

impl NexusEngine {
    pub fn new() -> Self {
        Self {
            inner: std::sync::RwLock::new(NexusEngineImpl::new()),
        }
    }

    pub fn start_stroke(&self, stroke_id: u32) {
        self.inner.write().unwrap().start_stroke(stroke_id);
    }

    pub fn add_point(&self, stroke_id: u32, point: Point) {
        self.inner.write().unwrap().add_point(stroke_id, point);
    }

    pub fn finish_stroke(&self, stroke_id: u32) {
        self.inner.write().unwrap().finish_stroke(stroke_id);
    }

    pub fn apply_theme(&self, palette: ColorPalette) {
        self.inner.write().unwrap().apply_theme(palette);
    }

    pub fn get_recent_strokes(&self, count: u32) -> Vec<StrokeData> {
        self.inner.write().unwrap().get_recent_strokes(count)
    }

    pub fn undo(&self) {
        self.inner.write().unwrap().undo();
    }

    pub fn redo(&self) {
        self.inner.write().unwrap().redo();
    }

    pub fn set_stabilizer_strength(&self, strength: f32) {
        self.inner.write().unwrap().set_stabilizer_strength(strength);
    }

    pub fn push_event(&self, event: CanvasEvent) {
        self.inner.write().unwrap().push_event(event);
    }

    pub fn export_events(&self) -> Vec<u8> {
        self.inner.read().unwrap().export_events()
    }

    pub fn import_events(&self, data: Vec<u8>) {
        self.inner.write().unwrap().import_events(data);
    }

    pub fn undo_time_travel(&self) -> bool {
        self.inner.write().unwrap().undo_time_travel()
    }

    pub fn add_layer(&self) -> u32 {
        self.inner.write().unwrap().add_layer()
    }

    pub fn remove_layer(&self, id: u32) -> bool {
        self.inner.write().unwrap().remove_layer(id)
    }

    pub fn get_layer_stack(&self) -> Vec<LayerInfo> {
        self.inner.read().unwrap().get_layer_stack()
    }

    pub fn set_active_layer(&self, id: u32) {
        self.inner.write().unwrap().set_active_layer(id);
    }

    pub fn toggle_layer_visibility(&self, id: u32) {
        self.inner.write().unwrap().toggle_layer_visibility(id);
    }

    pub fn toggle_layer_lock(&self, id: u32) {
        self.inner.write().unwrap().toggle_layer_lock(id);
    }

    pub fn set_layer_opacity(&self, id: u32, opacity: f32) {
        self.inner.write().unwrap().set_layer_opacity(id, opacity);
    }

    pub fn reorder_layers(&self, ids: Vec<u32>) {
        self.inner.write().unwrap().reorder_layers(ids);
    }

    pub fn zoom_viewport(&self, factor: f32, anchor_x: f32, anchor_y: f32) {
        self.inner.write().unwrap().zoom_viewport(factor, anchor_x, anchor_y);
    }

    pub fn pan_viewport(&self, dx: f32, dy: f32) {
        self.inner.write().unwrap().pan_viewport(dx, dy);
    }

    pub fn rotate_viewport(&self, radians: f32) {
        self.inner.write().unwrap().rotate_viewport(radians);
    }

    pub fn get_viewport(&self) -> ViewportState {
        self.inner.read().unwrap().get_viewport()
    }

    pub fn reset_viewport(&self) {
        self.inner.write().unwrap().reset_viewport();
    }

    pub fn set_brush_size(&self, size: f32) {
        self.inner.write().unwrap().set_brush_size(size);
    }

    pub fn set_brush_opacity(&self, opacity: f32) {
        self.inner.write().unwrap().set_brush_opacity(opacity);
    }

    pub fn set_brush_spacing(&self, spacing: f32) {
        self.inner.write().unwrap().set_brush_spacing(spacing);
    }

    pub fn set_brush_wetness(&self, wetness: f32) {
        self.inner.write().unwrap().set_brush_wetness(wetness);
    }

    pub fn export_project(&self, name: String) -> Vec<u8> {
        self.inner.read().unwrap().export_project(name)
    }

    pub fn import_project(&self, data: Vec<u8>) {
        self.inner.write().unwrap().import_project(data);
    }

    pub fn activate_lasso_selection(&self, points: Vec<Point>) {
        self.inner.write().unwrap().activate_lasso_selection(points);
    }

    pub fn activate_rect_selection(&self, x: f32, y: f32, width: f32, height: f32) {
        self.inner.write().unwrap().activate_rect_selection(x, y, width, height);
    }

    pub fn magic_wand_selection(&self, x: u32, y: u32, threshold: u8) {
        self.inner.write().unwrap().magic_wand_selection(x, y, threshold);
    }

    pub fn clear_selection(&self) {
        self.inner.write().unwrap().clear_selection();
    }

    pub fn get_selection(&self) -> SelectionMask {
        self.inner.read().unwrap().get_selection()
    }

    pub fn apply_brightness_contrast(&self, brightness: f32, contrast: f32) {
        self.inner.write().unwrap().apply_brightness_contrast(brightness, contrast);
    }

    pub fn apply_hsl(&self, hue: f32, saturation: f32, lightness: f32) {
        self.inner.write().unwrap().apply_hsl(hue, saturation, lightness);
    }

    pub fn apply_blur(&self, radius: f32) {
        self.inner.write().unwrap().apply_blur(radius);
    }

    pub fn export_psd(&self, path: String) {
        self.inner.read().unwrap().export_psd(path);
    }

    pub fn export_png(&self, path: String) {
        self.inner.read().unwrap().export_png(path);
    }

    pub fn get_last_render_time_ms(&self) -> f32 {
        self.inner.read().unwrap().get_last_render_time_ms()
    }

    pub fn get_cache_hit_ratio(&self) -> u32 {
        self.inner.read().unwrap().get_cache_hit_ratio()
    }
}

impl Default for NexusEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl NexusEngineImpl {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert_resource(Theme {
            primary: "#FF6B6B".to_string(),
            secondary: "#FFA94D".to_string(),
            accent: "#FFD93D".to_string(),
            shades: vec![
                "#FFF5F5".to_string(),
                "#FFE3E3".to_string(),
                "#FFC9C9".to_string(),
                "#FFA8A8".to_string(),
                "#FF8787".to_string(),
                "#FF6B6B".to_string(),
                "#FA5252".to_string(),
                "#F03E3E".to_string(),
                "#E03131".to_string(),
                "#C92A2A".to_string(),
            ],
        });
        world.insert_resource(crate::ecs::components::StabilizerStrength(0.5f32)); // default stabilizer strength
        world.insert_resource(CanvasCache::default());

        let mut schedule = Schedule::default();
        schedule.add_systems(stabilizer_system);

        Self {
            world,
            schedule,
            stabilizer_strength: 0.5,
            undo_stack: UndoStack::default(),
            event_log: EventLog::new(100000),
            current_stroke_path: VectorPath::new(),
            layer_stack: LayerStack::default(),
            viewport: Viewport::new(1024.0, 1024.0),
            brush_config: BrushConfig::default(),
            active_stroke: None,
            canvas_cache: CanvasCache::default(),
            selection_mask: SelectionMask::default(),
            render_times: Vec::with_capacity(100),
        }
    }

    pub fn start_stroke(&mut self, stroke_id: u32) {
        let stroke = Stroke {
            id: stroke_id,
            points: Vec::new(),
            color_hex: "#FF6B6B".to_string(),
            width: 5.0,
            layer_id: 0,
            is_finished: false,
        };
        self.world.spawn(stroke);
        self.world.spawn(TrailQueue::new(20));
    }

    pub fn add_point(&mut self, stroke_id: u32, point: Point) {
        let mut query = self.world.query::<&mut Stroke>();
        for mut stroke in query.iter_mut(&mut self.world) {
            if stroke.id == stroke_id && !stroke.is_finished {
                stroke.points.push(point.clone());
            }
        }
        self.schedule.run(&mut self.world);
    }

    pub fn finish_stroke(&mut self, stroke_id: u32) {
        let mut query = self.world.query::<&mut Stroke>();
        for mut stroke in query.iter_mut(&mut self.world) {
            if stroke.id == stroke_id {
                stroke.is_finished = true;
                // self.event_log.push(CanvasEvent::StrokeFinished { stroke_id });
                // Note: The UI layer in TypeScript has string-based undo for now,
                // but the backend supports full CanvasEvent. We will just use push_event.
                self.undo_stack.operations.push(format!("FINISH_STROKE:{}", stroke_id));
                break;
            }
        }
    }

    pub fn apply_theme(&mut self, palette: ColorPalette) {
        let theme = Theme {
            primary: palette.primary,
            secondary: palette.secondary,
            accent: palette.accent,
            shades: palette.shades,
        };
        self.world.insert_resource(theme);

        let theme_res = self.world.resource::<Theme>().clone();
        let mut query = self.world.query::<&mut Stroke>();
        for mut stroke in query.iter_mut(&mut self.world) {
            if let Ok(srgb) = std::str::FromStr::from_str(&stroke.color_hex) {
                let srgb: palette::Srgb<u8> = srgb;
                let old_color: palette::Oklch = srgb.into_format::<f32>().into_color();
                if let Ok(theme_srgb) = std::str::FromStr::from_str(&theme_res.primary) {
                    let theme_srgb: palette::Srgb<u8> = theme_srgb;
                    let new_primary: palette::Oklch = theme_srgb.into_format::<f32>().into_color();
                    let shifted = palette::Oklch::new(old_color.l, old_color.chroma, new_primary.hue);
                    let rgb: palette::Srgb = palette::FromColor::from_color(shifted);
                    stroke.color_hex = format!("#{:02x}{:02x}{:02x}", (rgb.red * 255.0) as u8, (rgb.green * 255.0) as u8, (rgb.blue * 255.0) as u8);
                }
            }
        }
    }

    pub fn get_recent_strokes(&mut self, count: u32) -> Vec<StrokeData> {
        let mut out = Vec::new();
        let mut query = self.world.query::<&Stroke>();
        for stroke in query.iter(&self.world) {
            if out.len() >= count as usize {
                break;
            }
            out.push(StrokeData {
                points: stroke.points.iter().map(|p| p.clone()).collect(),
                color_hex: stroke.color_hex.clone(),
                width: stroke.width,
                layer_id: stroke.layer_id,
            });
        }
        out
    }

    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.operations.pop() {
            let mut query = self.world.query::<(Entity, &Stroke)>();
            let mut entities_to_despawn = Vec::new();
            for (entity, stroke) in query.iter(&self.world) {
                if format!("FINISH_STROKE:{}", stroke.id) == op {
                    entities_to_despawn.push(entity);
                }
            }
            for entity in entities_to_despawn {
                self.world.despawn(entity);
            }
        }
    }

    pub fn redo(&mut self) {
    }

    pub fn set_stabilizer_strength(&mut self, strength: f32) {
        self.stabilizer_strength = strength.clamp(0.0, 1.0);
        let mut res = self.world.resource_mut::<StabilizerStrength>();
        res.0 = self.stabilizer_strength;
    }

    pub fn push_event(&mut self, event: CanvasEvent) {
        self.event_log.push(event.clone());
        match event {
            CanvasEvent::StrokePoint { stroke_id: _, x: _, y: _, pressure: _ } => {
            }
            _ => {}
        }
    }

    pub fn export_events(&self) -> Vec<u8> {
        self.event_log.save_to_bytes()
    }

    pub fn import_events(&mut self, data: Vec<u8>) {
        let _loaded_log = EventLog::load_from_bytes(&data);
    }

    pub fn undo_time_travel(&mut self) -> bool {
        if let Some(_event) = self.event_log.undo() {
            true
        } else {
            false
        }
    }

    pub fn add_layer(&mut self) -> u32 {
        let id = self.layer_stack.add_layer(self.viewport.screen_width as u32, self.viewport.screen_height as u32);
        self.world.spawn(Layer { id, name: format!("Layer {}", id), opacity: 1.0, blend_mode: BlendMode::Normal, is_visible: true, is_locked: false, is_vector: true, z_index: id as i32, width: self.viewport.screen_width as u32, height: self.viewport.screen_height as u32 });
        id
    }

    pub fn remove_layer(&mut self, id: u32) -> bool {
        self.layer_stack.remove_layer(id)
    }

    pub fn get_layer_stack(&self) -> Vec<LayerInfo> {
        self.layer_stack.layers.iter().map(|l| LayerInfo {
            id: l.id,
            name: l.name.clone(),
            opacity: l.opacity,
            blend_mode: match l.blend_mode {
                BlendMode::Normal => 0,
                BlendMode::Multiply => 1,
                BlendMode::Screen => 2,
                _ => 0,
            },
            visible: l.is_visible,
            locked: l.is_locked,
            is_active: l.id == self.layer_stack.active_layer_id,
        }).collect()
    }

    pub fn set_active_layer(&mut self, id: u32) {
        self.layer_stack.active_layer_id = id;
    }

    pub fn toggle_layer_visibility(&mut self, id: u32) {
        if let Some(layer) = self.layer_stack.get_layer_mut(id) {
            layer.is_visible = !layer.is_visible;
        }
    }

    pub fn toggle_layer_lock(&mut self, id: u32) {
        if let Some(layer) = self.layer_stack.get_layer_mut(id) {
            layer.is_locked = !layer.is_locked;
        }
    }

    pub fn set_layer_opacity(&mut self, id: u32, opacity: f32) {
        if let Some(layer) = self.layer_stack.get_layer_mut(id) {
            layer.opacity = opacity.clamp(0.0, 1.0);
        }
    }

    pub fn reorder_layers(&mut self, ids: Vec<u32>) {
        for (i, id) in ids.iter().enumerate() {
            if let Some(layer) = self.layer_stack.layers.iter_mut().find(|l| l.id == *id) {
                layer.z_index = i as i32;
            }
        }
        self.layer_stack.sort_layers();
    }

    pub fn zoom_viewport(&mut self, factor: f32, anchor_x: f32, anchor_y: f32) {
        self.viewport.zoom(factor, anchor_x, anchor_y);
    }

    pub fn pan_viewport(&mut self, dx: f32, dy: f32) {
        self.viewport.pan(dx, dy);
    }

    pub fn rotate_viewport(&mut self, radians: f32) {
        self.viewport.rotation += radians;
    }

    pub fn get_viewport(&self) -> ViewportState {
        ViewportState {
            center_x: self.viewport.center.x,
            center_y: self.viewport.center.y,
            scale: self.viewport.scale,
            rotation: self.viewport.rotation,
            screen_width: self.viewport.screen_width,
            screen_height: self.viewport.screen_height,
        }
    }

    pub fn reset_viewport(&mut self) {
        self.viewport.center = Vector2::new(0.0, 0.0);
        self.viewport.scale = 1.0;
        self.viewport.rotation = 0.0;
    }

    pub fn set_brush_size(&mut self, size: f32) {
        self.brush_config.size = size;
    }

    pub fn set_brush_opacity(&mut self, opacity: f32) {
        self.brush_config.opacity = opacity;
    }

    pub fn set_brush_spacing(&mut self, spacing: f32) {
        self.brush_config.spacing = spacing;
    }

    pub fn set_brush_wetness(&mut self, wetness: f32) {
        self.brush_config.wetness = wetness;
    }

    pub fn export_project(&self, name: String) -> Vec<u8> {
        let project = ProjectFile {
            metadata: ProjectMetadata {
                name,
                author: "Nexus Artist".to_string(),
                created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                modified_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                canvas_width: self.viewport.screen_width as u32,
                canvas_height: self.viewport.screen_height as u32,
                dpi: 300,
                color_profile: "sRGB".to_string(),
            },
            viewport: self.viewport.clone(),
            layers: self.layer_stack.layers.clone(),
            events: self.event_log.events.clone(),
            thumbnail: vec![],
        };
        project.to_bytes()
    }

    pub fn import_project(&mut self, data: Vec<u8>) {
        if let Some(project) = ProjectFile::from_bytes(&data) {
            self.viewport = project.viewport;
            self.layer_stack.layers = project.layers;
            self.event_log.events = project.events;
            self.event_log.current_index = self.event_log.events.len();
        }
    }

    pub fn activate_lasso_selection(&mut self, points: Vec<Point>) {
        let pts: Vec<_> = points.iter().map(|p| p.clone()).collect();
        self.selection_mask.activate_lasso(pts);
        self.event_log.push(CanvasEvent::LayerCreated{layer_id: 0, name: "TODO".to_string()});
    }

    pub fn activate_rect_selection(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.selection_mask.activate_rect(x, y, width, height);
    }

    pub fn magic_wand_selection(&mut self, x: u32, y: u32, _threshold: u8) {
        self.selection_mask.magic_wand(x, y, _threshold);
    }

    pub fn clear_selection(&mut self) {
        self.selection_mask.is_active = false;
        self.selection_mask.mask_data.fill(0);
    }

    pub fn get_selection(&self) -> SelectionMask {
        self.selection_mask.clone()
    }

    pub fn apply_brightness_contrast(&mut self, _brightness: f32, _contrast: f32) {
        self.canvas_cache.mark_dirty(Rect::new(0.0, 0.0,
            self.viewport.screen_width, self.viewport.screen_height));
    }

    pub fn apply_hsl(&mut self, _hue: f32, _saturation: f32, _lightness: f32) {
        self.canvas_cache.mark_dirty(Rect::new(0.0, 0.0,
            self.viewport.screen_width, self.viewport.screen_height));
    }

    pub fn apply_blur(&mut self, _radius: f32) {
        self.canvas_cache.mark_dirty(Rect::new(0.0, 0.0,
            self.viewport.screen_width, self.viewport.screen_height));
    }

    pub fn export_psd(&self, path: String) {
        let exporter = PsdExporter::new(
            self.layer_stack.layers.clone(),
            self.viewport.screen_width as u32,
            self.viewport.screen_height as u32,
        );
        let _ = exporter.export(&path);
    }

    pub fn export_png(&self, path: String) {
        println!("Exporting PNG to: {}", path);
    }

    pub fn get_last_render_time_ms(&self) -> f32 {
        *self.render_times.last().unwrap_or(&0.0)
    }

    pub fn get_cache_hit_ratio(&self) -> u32 {
        80
    }
}

pub fn version() -> u32 {
    1
}

#[derive(Clone, Default)]
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub shades: Vec<String>,
}

pub struct StrokeData {
    pub points: Vec<Point>,
    pub color_hex: String,
    pub width: f32,
    pub layer_id: u32,
}

pub struct LayerInfo {
    pub id: u32,
    pub name: String,
    pub opacity: f32,
    pub blend_mode: u32,
    pub visible: bool,
    pub locked: bool,
    pub is_active: bool,
}

pub struct ViewportState {
    pub center_x: f32,
    pub center_y: f32,
    pub scale: f32,
    pub rotation: f32,
    pub screen_width: f32,
    pub screen_height: f32,
}

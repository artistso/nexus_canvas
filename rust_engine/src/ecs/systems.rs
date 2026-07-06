use bevy_ecs::prelude::*;
use nalgebra::Vector2;
use palette::{FromColor, Hsl, Oklch, Srgb};
use crate::ecs::components::*;

pub fn stabilizer_system(
    mut stroke_query: Query<(&mut Stroke, &mut TrailQueue)>,
    stabilizer_strength: Res<f32>,
) {
    for (mut stroke, mut trail) in stroke_query.iter_mut() {
        if stroke.points.len() < 3 {
            continue;
        }

        let len = stroke.points.len();
        let last = stroke.points[len - 1];
        let prev = stroke.points[len - 2];

        // Weighted moving average (Kalman-style)
        let weight = *stabilizer_strength;
        let smoothed_x = prev.x * (1.0 - weight) + last.x * weight;
        let smoothed_y = prev.y * (1.0 - weight) + last.y * weight;

        stroke.points[len - 1] = Point {
            x: smoothed_x,
            y: smoothed_y,
            ..last
        };

        // Push to trail queue
        trail.push(last);
    }
}

pub fn trail_generator_system(
    trail_query: Query<&TrailQueue>,
) -> Vec<(f32, f32, f32, f32)> {
    let mut out = Vec::new();
    for trail in trail_query.iter() {
        let count = trail.points.len();
        for (i, point) in trail.points.iter().enumerate() {
            let progress = i as f32 / count as f32;
            let alpha = progress; // older = more transparent
            let radius = 5.0 + progress * 15.0; // grows toward tip
            out.push((point.x, point.y, radius, alpha));
        }
    }
    out
}

pub fn theme_apply_system(
    mut stroke_query: Query<&mut Stroke>,
    theme: Res<Theme>,
) {
    for mut stroke in stroke_query.iter_mut() {
        // Map the stroke's existing color to the new theme
        // Simple approach: replace with primary if it matched the old primary
        // Advanced: OKLCH hue shift based on delta
        let old_color = Oklch::from(Srgb::from_hex(&stroke.color_hex).unwrap());
        let new_primary = Oklch::from(Srgb::from_hex(&theme.primary).unwrap());

        // Shift hue to match theme, keep lightness/chroma
        let shifted = Oklch::new(
            old_color.l,
            old_color.chroma,
            new_primary.hue,
        );
        let rgb: Srgb = Srgb::from_color(shifted);
        stroke.color_hex = format!("#{:02x}{:02x}{:02x}",
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
        );
    }
}

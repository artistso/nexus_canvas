// Liquify Compute Shader (WebGPU/WGSL)
// Pulls pixels based on a displacement grid.

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var displacement_grid: texture_2d<f32>;
@group(0) @binding(2) var result: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let coord = vec2<i32>(global_id.xy);
    let dims = textureDimensions(texture);

    if (coord.x >= dims.x || coord.y >= dims.y) {
        return;
    }

    // Read displacement vector from grid
    let displacement = textureLoad(displacement_grid, coord, 0).xy * 10.0; // Scale factor

    // Sample source texture at displaced coordinate
    let source_coord = vec2<f32>(coord) + displacement;
    let clamped_coord = vec2<i32>(clamp(source_coord, vec2<f32>(0.0), vec2<f32>(dims - 1)));

    let color = textureLoad(texture, clamped_coord, 0);
    textureStore(result, coord, color);
}

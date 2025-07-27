// Solid color shader for UI elements

struct UiUniforms {
    view_proj: mat4x4<f32>,
    screen_size: vec2<f32>,
}

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: UiUniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Convert screen coordinates to normalized device coordinates
    let screen_pos = vec4<f32>(input.position, 0.0, 1.0);
    output.clip_position = uniforms.view_proj * screen_pos;
    
    output.color = input.color;
    output.tex_coords = input.tex_coords;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
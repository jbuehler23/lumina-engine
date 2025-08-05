// Textured shader for UI elements

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

@group(1) @binding(0)
var texture_sampler: sampler;

@group(1) @binding(1)
var texture_data: texture_2d<f32>;

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
    // Sample from R8Unorm font atlas - use red channel as alpha for text rendering
    let alpha = textureSample(texture_data, texture_sampler, input.tex_coords).r;
    // Return color with alpha modulated by glyph coverage
    return vec4<f32>(input.color.rgb, input.color.a * alpha);
}
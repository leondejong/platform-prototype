// Vertex shader

struct VertexInput {
    @location(0) vertex_coordinates: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
}

struct VertexOutput {
    @location(0) texture_coordinates: vec2<f32>,
    @builtin(position) clip_coordinates: vec4<f32>,
}

@vertex
fn vs_main(
    input: VertexInput,
) -> VertexOutput {
    var output: VertexOutput;
    output.texture_coordinates = input.texture_coordinates;
    output.clip_coordinates = vec4<f32>(input.vertex_coordinates, 1.0);
    return output;
}

// Fragment shader

@group(0) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(0) @binding(1)
var diffuse_sampler: sampler;

@fragment
fn fs_main(output: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(diffuse_texture, diffuse_sampler, output.texture_coordinates);
}

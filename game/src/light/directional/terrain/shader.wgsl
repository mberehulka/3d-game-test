struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
};

struct DirectionalLight {
    @location(0) projection: mat4x4<f32>,
    @location(1) direction: vec4<f32>
};
@group(0) @binding(0)
var<uniform> dir_light: DirectionalLight;

struct Output {
    @builtin(position) position: vec4<f32>
};

@vertex
fn vs_main(vertex: Vertex) -> Output {
    var out: Output;
    out.position = dir_light.projection * vec4<f32>(vertex.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return vec4<f32>(vec3<f32>(in.position.w), 1.0);
}
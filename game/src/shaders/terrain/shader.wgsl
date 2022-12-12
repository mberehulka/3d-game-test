struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
};

struct Camera {
    @location(0) projection: mat4x4<f32>,
    @location(1) position: vec4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Sun {
    @location(0) projection: mat4x4<f32>,
    @location(1) direction: vec4<f32>
};
@group(2) @binding(0)
var<uniform> sun: Sun;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) position_sun_space: vec4<f32>,
    @location(1) uv: vec2<f32>
};

@vertex
fn vs_main(vertex: Vertex) -> Output {
    var out: Output;
    out.uv = vertex.uv;
    let pos = vec4<f32>(vertex.position, 1.0);
    out.position = camera.projection * pos;
    out.position_sun_space = sun.projection * pos;
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1)@binding(1)
var s_diffuse: sampler;

@group(2) @binding(1)
var sun_texture: texture_2d<f32>;
@group(2)@binding(2)
var sun_texture_sampler: sampler;

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    let texture = textureSample(t_diffuse, s_diffuse, in.uv);
    //shadow
    let closest_depth = textureSample(sun_texture, sun_texture_sampler, vec2<f32>(
        ((in.position_sun_space.x / in.position_sun_space.w) + 1.0) * 0.5,
        1.0-(((in.position_sun_space.y / in.position_sun_space.w) + 1.0) * 0.5)
    )).r;
    let current_depth = 1.011 - (((in.position_sun_space.z / in.position_sun_space.w) + 1.0) * 0.5);
    var shadow = 0.5;
    if(current_depth > closest_depth){ shadow = 1.0; }
    return texture * shadow;
}
struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>
};

struct Camera {
    @location(0) projection: mat4x4<f32>,
    @location(1) position: vec4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Sun {
    @location(0) projection: mat4x4<f32>,
    @location(1) biased_projection: mat4x4<f32>,
    @location(2) direction: vec4<f32>
};
@group(2) @binding(0)
var<uniform> sun: Sun;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) position_sun_space: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>
};

@vertex
fn vs_main(vertex: Vertex) -> Output {
    var out: Output;
    out.uv = vertex.uv;
    let pos = vec4<f32>(vertex.position, 1.0);
    out.position = camera.projection * pos;
    out.position_sun_space = sun.biased_projection * pos;
    out.normal = vertex.normal;
    return out;
}

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(1)@binding(1)
var diffuse_texture_sampler: sampler;

@group(2) @binding(1)
var sun_texture: texture_2d<f32>;
@group(2)@binding(2)
var sun_texture_sampler: sampler;

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    let texture = textureSample(diffuse_texture, diffuse_texture_sampler, in.uv);
    //shadow
    let closest_depth = textureSample(sun_texture, sun_texture_sampler, vec2<f32>(
        in.position_sun_space.x * 0.5 + 0.5,
        in.position_sun_space.y * -0.5 + 0.5
    )).r;
    let bias = clamp(tan(acos( dot(in.normal,sun.direction.xyz)*0.5+0.5 )), 0., 0.01);
    var shadow = 1.0;
    if(closest_depth < in.position_sun_space.z - bias){{ shadow = 0.85; }}
    return texture * shadow;
}
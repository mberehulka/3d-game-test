struct Vertex {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>
};

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>
};

@vertex
fn vs_main(vertex: Vertex) -> Output {
    var out: Output;
    out.uv = vertex.uv;
    out.color = vertex.color;
    out.position = vec4<f32>(vertex.position.x, vertex.position.y, 0.0, 0.5);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uv) * in.color;
}
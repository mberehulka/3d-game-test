struct Vertex {
    @location(0) position: vec3<f32>,
    @location(3) joints: vec4<u32>,
    @location(4) weights: vec4<f32>
};
struct Transform {
    @location(5) position: vec3<f32>,
    @location(6) scale: vec3<f32>
};

struct DirectionalLight {
    @location(0) projection: mat4x4<f32>,
    @location(1) biased_projection: mat4x4<f32>,
    @location(2) direction: vec4<f32>
};
@group(0) @binding(0)
var<uniform> dir_light: DirectionalLight;

struct Skin {
    @location(0) pose: array<mat4x4<f32>,128>
};
@group(1) @binding(0)
var<uniform> skin: Skin;

struct Output {
    @builtin(position) position: vec4<f32>
};

fn apply_skin(vertex: Vertex, v3: vec3<f32>) -> vec3<f32> {
    let v4 = vec4<f32>(v3, 1.0);
    var res = ((skin.pose[vertex.joints[0]] * v4) * vertex.weights[0]);
    if (vertex.joints[1] != 255u) { res += ((skin.pose[vertex.joints[1]] * v4) * vertex.weights[1]); }
    if (vertex.joints[2] != 255u) { res += ((skin.pose[vertex.joints[2]] * v4) * vertex.weights[2]); }
    if (vertex.joints[3] != 255u) { res += ((skin.pose[vertex.joints[3]] * v4) * vertex.weights[3]); }
    return res.xyz;
}

@vertex
fn vs_main(vertex: Vertex, transform: Transform) -> Output {
    var out: Output;
    out.position = dir_light.projection * vec4<f32>((apply_skin(vertex, vertex.position) * transform.scale), 1.0);
    return out;
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return vec4<f32>(vec3<f32>(in.position.z), 1.0);
}
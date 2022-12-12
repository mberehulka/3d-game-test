mod material;

pub use material::Material;
use wgpu::ShaderModuleDescriptor;

use crate::{assets::{InstanceTransform, DEPTH_FORMAT}, light::directional::directional_light_bind_group_layout};

pub struct Shader {
    pub render_pipeline: wgpu::RenderPipeline
}

impl Shader {
    pub fn new(device: &wgpu::Device, surface_texture_format: wgpu::TextureFormat, a: f32, b: f32, c: f32) -> Self {
        log::info!("Creating basic_anim shader");
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(format!("
struct Vertex {{
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) joints: vec4<u32>,
    @location(4) weights: vec4<f32>
}};
struct Transform {{
    @location(5) position: vec3<f32>,
    @location(6) scale: vec3<f32>
}};

struct Camera {{
    @location(0) projection: mat4x4<f32>,
    @location(1) position: vec4<f32>
}};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct SkinPoses {{
    @location(0) mats: array<mat4x4<f32>,128>
}};
@group(2) @binding(0)
var<uniform> skin_poses: SkinPoses;
struct SkinRotations {{
    @location(0) mats: array<mat4x4<f32>,128>
}};
@group(2) @binding(1)
var<uniform> skin_rotations: SkinPoses;

struct Sun {{
    @location(0) projection: mat4x4<f32>,
    @location(1) direction: vec4<f32>
}};
@group(3) @binding(0)
var<uniform> sun: Sun;

struct Output {{
    @builtin(position) position: vec4<f32>,
    @location(0) position_sun_space: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>
}};

fn apply_skin_pose(vertex: Vertex, v3: vec3<f32>) -> vec3<f32> {{
    let v4 = vec4<f32>(v3, 1.0);
    var res = (skin_poses.mats[vertex.joints[0]] * v4) * vertex.weights[0];
    if (vertex.joints[1] != 255u) {{ res += (skin_poses.mats[vertex.joints[1]] * v4) * vertex.weights[1]; }}
    if (vertex.joints[2] != 255u) {{ res += (skin_poses.mats[vertex.joints[2]] * v4) * vertex.weights[2]; }}
    if (vertex.joints[3] != 255u) {{ res += (skin_poses.mats[vertex.joints[3]] * v4) * vertex.weights[3]; }}
    return res.xyz;
}}
fn apply_skin_rotation(vertex: Vertex, v3: vec3<f32>) -> vec3<f32> {{
    let v4 = vec4<f32>(v3, 1.0);
    var res = (skin_rotations.mats[vertex.joints[0]] * v4) * vertex.weights[0];
    if (vertex.joints[1] != 255u) {{ res += (skin_rotations.mats[vertex.joints[1]] * v4) * vertex.weights[1]; }}
    if (vertex.joints[2] != 255u) {{ res += (skin_rotations.mats[vertex.joints[2]] * v4) * vertex.weights[2]; }}
    if (vertex.joints[3] != 255u) {{ res += (skin_rotations.mats[vertex.joints[3]] * v4) * vertex.weights[3]; }}
    return res.xyz;
}}

@vertex
fn vs_main(vertex: Vertex, transform: Transform) -> Output {{
    var out: Output;
    out.uv = vertex.uv;
    let pos = vec4<f32>((apply_skin_pose(vertex, vertex.position) * transform.scale), 1.0);
    out.position = camera.projection * pos;
    out.position_sun_space = sun.projection * pos;
    out.normal = normalize(apply_skin_rotation(vertex, vertex.normal));
    return out;
}}

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(1) @binding(1)
var diffuse_texture_sampler: sampler;

@group(3) @binding(1)
var sun_texture: texture_2d<f32>;
@group(3) @binding(2)
var sun_texture_sampler: sampler;

fn sample(x: f32, y: f32, w: f32) -> f32 {{
    return textureSample(sun_texture, sun_texture_sampler, vec2<f32>(
        ((x / w) + 1.0) * 0.5,
        1.0-(((y / w) + 1.0) * 0.5)
    )).r;
}}

let ss: f32 = {a:.9};

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {{
    let texture = textureSample(diffuse_texture, diffuse_texture_sampler, in.uv);
    let normal = dot(in.normal, sun.direction.xyz) * 0.5 + 0.5;
    let closest_depth = 1.0 - (
        (
            sample(in.position_sun_space.x     , in.position_sun_space.y     , in.position_sun_space.w) +
            sample(in.position_sun_space.x + ss, in.position_sun_space.y     , in.position_sun_space.w) +
            sample(in.position_sun_space.x     , in.position_sun_space.y + ss, in.position_sun_space.w) +
            sample(in.position_sun_space.x + ss, in.position_sun_space.y + ss, in.position_sun_space.w) +
            sample(in.position_sun_space.x - ss, in.position_sun_space.y     , in.position_sun_space.w) +
            sample(in.position_sun_space.x     , in.position_sun_space.y - ss, in.position_sun_space.w) +
            sample(in.position_sun_space.x - ss, in.position_sun_space.y - ss, in.position_sun_space.w) +
            sample(in.position_sun_space.x + ss, in.position_sun_space.y - ss, in.position_sun_space.w) +
            sample(in.position_sun_space.x - ss, in.position_sun_space.y + ss, in.position_sun_space.w)
        ) / 9.0
    );
    let bias = clamp(tan(acos(1.0 - normal)), {b:.9}, {c:.9});
    let current_depth = ((in.position_sun_space.z / in.position_sun_space.w) + 1.0) * 0.5;
    var shadow = 1.0;
    if(closest_depth < current_depth - bias || normal < 0.1){{
        shadow = 0.85;
    }} else if(normal > 0.9){{
        shadow = 1.15;
    }}
    return vec4<f32>(texture * shadow);
}}
").into())
        });
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Basic animation shader render pipeline layout"),
            bind_group_layouts: &[
                &crate::camera::bind_group_layout(device),
                &crate::assets::texture_bind_group(device),
                &crate::assets::armature_bind_group_layout(device),
                &directional_light_bind_group_layout(device)
            ],
            push_constant_ranges: &[]
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Basic animation shader render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    crate::assets::vertex::VertexNUS::LAYOUT,
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<InstanceTransform>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![5 => Float32x3, 6 => Float32x3]
                    }
                ]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_texture_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default()
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });
        Self {
            render_pipeline
        }
    }
}
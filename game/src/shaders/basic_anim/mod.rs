mod material;

pub use material::Material;
use wgpu::ShaderModuleDescriptor;

use crate::{assets::{InstanceTransform, DEPTH_FORMAT}, light::directional::directional_light_bind_group_layout};

pub struct Shader {
    pub render_pipeline: wgpu::RenderPipeline
}

impl Shader {
    pub fn new(device: &wgpu::Device, surface_texture_format: wgpu::TextureFormat) -> Self {
        log::info!("Creating basic_anim shader");
        let a = 0.001;
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
    @location(1) biased_projection: mat4x4<f32>,
    @location(2) direction: vec4<f32>
}};
@group(3) @binding(0)
var<uniform> sun: Sun;

struct Output {{
    @builtin(position) position: vec4<f32>,
    @location(0) position_sun_space: vec4<f32>,
    @location(1) vertex_position: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) uv: vec2<f32>
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
    out.vertex_position = apply_skin_pose(vertex, vertex.position) * transform.scale;
    let pos = vec4<f32>(out.vertex_position, 1.0);
    out.position = camera.projection * pos;
    out.position_sun_space = sun.biased_projection * pos;
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

fn get_closest_depth(x: f32, y: f32) -> f32 {{
    return textureSample(sun_texture, sun_texture_sampler, vec2<f32>(
        x * 0.5 + 0.5,
        y * -0.5 + 0.5
    )).r;
}}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {{
    let texture = textureSample(diffuse_texture, diffuse_texture_sampler, in.uv);
    let closest_depth = (
        get_closest_depth(in.position_sun_space.x      , in.position_sun_space.y      ) +
        get_closest_depth(in.position_sun_space.x + {a}, in.position_sun_space.y      ) +
        get_closest_depth(in.position_sun_space.x      , in.position_sun_space.y + {a}) +
        get_closest_depth(in.position_sun_space.x + {a}, in.position_sun_space.y + {a}) +
        get_closest_depth(in.position_sun_space.x - {a}, in.position_sun_space.y      ) +
        get_closest_depth(in.position_sun_space.x      , in.position_sun_space.y - {a}) +
        get_closest_depth(in.position_sun_space.x - {a}, in.position_sun_space.y - {a}) +
        get_closest_depth(in.position_sun_space.x + {a}, in.position_sun_space.y - {a}) +
        get_closest_depth(in.position_sun_space.x - {a}, in.position_sun_space.y + {a})
    ) / 9.0;
    let ndotl = dot(in.normal, sun.direction.xyz);
    let view_dir = normalize(camera.position.xyz - in.vertex_position);
    let reflect_dir = reflect(-sun.direction.xyz, in.normal);

    let bias = clamp(tan(acos( ndotl*0.5+0.5 )), 0., 0.01);
    var light = 1.0;
    if(closest_depth < in.position_sun_space.z - bias){{ light -= 0.15; }}

    let specular = dot(view_dir, reflect_dir);
    if(specular > 0.5){{ light += 0.1; }}

    let rim = 1.0 - dot(view_dir, in.normal);
    if(rim > 0.6){{ light += 0.2; }}

    return texture * light;
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
use crate::assets::{InstanceTransform, DEPTH_FORMAT, DEFAULT_FORMAT};

pub struct Shader {
    pub render_pipeline: wgpu::RenderPipeline
}
impl Shader {
    pub fn new(device: &wgpu::Device) -> Self {
        log::info!("Creating basic_anim directional light shader");
        let shader = device.create_shader_module(wgpu::include_wgsl!("./shader.wgsl").into());
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("basic_anim directional light shader render pipeline"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("basic_anim directional light shader render pipeline layout"),
                bind_group_layouts: &[
                    &super::directional_light_bind_group_layout(device),
                    &crate::assets::armature_bind_group_layout(device)
                ],
                push_constant_ranges: &[]
            })),
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
                    format: DEFAULT_FORMAT,
                    blend: None,
                    write_mask: wgpu::ColorWrites::RED
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
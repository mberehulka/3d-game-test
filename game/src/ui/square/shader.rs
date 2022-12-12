use wgpu::BlendComponent;

pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
    log::info!("Creating square shader");
    let shader = device.create_shader_module(wgpu::include_wgsl!("./shader.wgsl").into());
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("square shader render pipeline layout"),
        bind_group_layouts: &[
            &crate::assets::texture_bind_group(device)
        ],
        push_constant_ranges: &[]
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("square shader render pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                super::SquareVertex::LAYOUT
            ]
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState { color: BlendComponent::OVER, alpha: BlendComponent::REPLACE }),
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
            format: crate::assets::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Always,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default()
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        },
        multiview: None
    })
}
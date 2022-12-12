use std::sync::{atomic::AtomicBool, Mutex};
use cgmath::{Matrix4, Quaternion, Deg, Rotation3, Vector3, InnerSpace};
use wgpu::{util::DeviceExt, Queue, TextureUsages};

use crate::{assets::{Texture, DEFAULT_FORMAT}, context::Context, shaders::Material};

pub mod basic_anim;
pub mod terrain;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DirectionalLightBinding {
    pub projection: [[f32;4];4],
    pub biased_projection: [[f32;4];4],
    pub direction: [f32;4]
}

pub struct DirectionalLight {
    pub basic_anim: basic_anim::Shader,
    pub terrain: terrain::Shader,

    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub depth_texture: Texture,
    pub output_texture: Texture,
    pub shadow_texture: Texture,

    pub direction: Mutex<[f32;3]>,
    pub size: Mutex<f32>,
    pub needs_update: AtomicBool
}
impl DirectionalLight {
    pub fn new(
        device: &wgpu::Device,
        direction: [f32;3],
        size: f32,
        texture_size: u32
    ) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[Self::get_binding(direction, size)]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        let output_texture = Texture::blank("Directional light output texture", device,
            texture_size, texture_size, DEFAULT_FORMAT,
            TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC);
        let shadow_texture = Texture::blank("Directional light output texture", device,
            texture_size, texture_size, DEFAULT_FORMAT,
            TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &directional_light_bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&shadow_texture.view)
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&shadow_texture.sampler)
                }
            ]
        });
        Self {
            buffer, bind_group, shadow_texture, output_texture,
            
            basic_anim: basic_anim::Shader::new(device),
            terrain: terrain::Shader::new(device),

            depth_texture: Texture::depth("Directional light depth texture", device, texture_size, texture_size),
            direction: Mutex::new(direction),
            size: Mutex::new(size),
            needs_update: AtomicBool::new(false)
        }
    }
    pub fn get_binding(direction: [f32;3], s: f32) -> DirectionalLightBinding {
        let perspective = cgmath::ortho(-s, s, -s, s, -s, s);
        let rotation = Quaternion::from_angle_x(Deg(direction[0])) *
                       Quaternion::from_angle_y(Deg(direction[1]));
        let distance = Matrix4::from_translation([0.,0.,-s/2.].into());
        let projection = perspective * (distance * Matrix4::from(rotation));
        let direction = (
            Quaternion::from_angle_x(Deg(-direction[0])) *
            Quaternion::from_angle_y(Deg(-direction[1])) *
            Vector3::new(0.,0.,1.)
        ).normalize();
        DirectionalLightBinding {
            projection: projection.into(),
            biased_projection: (projection).into(),
            direction: [direction.x, direction.y, direction.z, 1.]
        }
    }
    pub fn update(&self, queue: &Queue) {
        if !self.needs_update.load(std::sync::atomic::Ordering::Relaxed) { return }
        self.needs_update.store(false, std::sync::atomic::Ordering::Relaxed);
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[
            Self::get_binding(*self.direction.lock().unwrap(), *self.size.lock().unwrap())
        ]))
    }
    pub fn draw(&self, c: &Context) {
        let mut encoder = c.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let objects = &c.objects.0.lock().unwrap();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.output_texture.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true
                    }),
                    stencil_ops: None
                })
            });
            for object in objects.iter() {
                object.update(&c.queue);
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.set_vertex_buffer(0, object.mesh.vertices_buffer.slice(..));
                render_pass.set_vertex_buffer(1, object.instances.buffer.slice(..));
                match &object.material {
                    Material::BasicAnim(_) => {
                        render_pass.set_pipeline(&self.basic_anim.render_pipeline);
                        render_pass.set_bind_group(1, &object.armature.as_ref().unwrap().bind_group, &[]);
                        render_pass.draw(0..object.mesh.vertices_len, 0..object.instances.get_buffer_len());
                    }
                    Material::Terrain(_) => {
                        render_pass.set_pipeline(&self.terrain.render_pipeline);
                        render_pass.draw(0..object.mesh.vertices_len, 0..1);
                    }
                }
            }
        }
        encoder.copy_texture_to_texture(
            self.output_texture.texture.as_image_copy(), self.shadow_texture.texture.as_image_copy(), self.shadow_texture.size);
        c.queue.submit(std::iter::once(encoder.finish()));
    }
    pub fn rotate(&self, x: f32, y: f32, z: f32) {
        let mut direction = self.direction.lock().unwrap();
        direction[0] += x;
        direction[1] += y;
        direction[2] += z;
        self.needs_update.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

pub fn directional_light_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None
            }
        ]
    })
}
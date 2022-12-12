pub mod shader;

use std::sync::Arc;

use wgpu::util::DeviceExt;

use super::UIElementTexture;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SquareVertex {
    pub position: [f32;2],
    pub uv: [f32;2],
    pub color: [f32;4]
}
impl SquareVertex {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4]
    };
}

pub struct Square {
    pub buffer: wgpu::Buffer,
    pub texture: UIElementTexture
}
impl Square {
    pub fn new(
        device: &wgpu::Device,
        texture: impl Into<UIElementTexture>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [[f32;4];4]
    ) -> Arc<Self> {
        let vertices = Self::get_vertices(x, y, width, height, color);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX
        });
        Arc::new(Self {
            buffer, texture: texture.into()
        })
    }
    pub fn get_vertices(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [[f32;4];4]
    ) -> [SquareVertex;6] {
        let w2 = width / 2.;
        let h2 = height / 2.;
        let tl = SquareVertex {
            position: [x - w2, y - h2],
            uv: [0.,1.],
            color: color[0]
        };
        let tr = SquareVertex {
            position: [x - w2, y + h2],
            uv: [0.,0.],
            color: color[1]
        };
        let bl = SquareVertex {
            position: [x + w2, y - h2],
            uv: [1.,1.],
            color: color[2]
        };
        let br = SquareVertex {
            position: [x + w2, y + h2],
            uv: [1.,0.],
            color: color[3]
        };
        [tl, bl, br, tr, tl, br]
    }
}
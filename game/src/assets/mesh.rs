use wgpu::util::DeviceExt;

use super::{Reader, VertexType, Joint, VertexNUS, VertexU};

pub struct Mesh {
    pub name: String,
    pub vertex_type: VertexType,
    pub vertices_buffer: wgpu::Buffer,
    pub vertices_len: u32,
    pub joints: Vec<Joint>
}
impl Mesh {
    pub fn load(device: &wgpu::Device, reader: &mut Reader) -> Self {
        let name = reader.read_string();
        let vertex_type = VertexType::read(reader);
    
        let mut vertices_len = 0;
        let vertices_buffer = match vertex_type {
            VertexType::U => {
                get_vertices_buffer::<VertexU>(device, reader, &mut vertices_len, |reader| {
                    VertexU {
                        position: reader.read_vec3(),
                        uv: reader.read_vec2()
                    }
                })
            }
            VertexType::NUS => {
                get_vertices_buffer::<VertexNUS>(device, reader, &mut vertices_len, |reader| {
                    VertexNUS {
                        position: reader.read_vec3(),
                        normal: reader.read_vec3(),
                        uv: reader.read_vec2(),
                        joints: reader.read_joints(),
                        weights: reader.read_vec4()
                    }
                })
            }
        };

        let joints = match vertex_type {
            VertexType::NUS => Joint::read(reader),
            _ => Vec::new()
        };
        
        reader.read_end(&name);

        Self {
            name,
            vertex_type,
            vertices_buffer,
            vertices_len,
            joints
        }
    }
}

#[inline]
fn get_vertices_buffer<V: Copy + Clone + bytemuck::Pod + bytemuck::Zeroable>(
    device: &wgpu::Device,
    reader: &mut Reader,
    vertices_len: &mut u32,
    f: fn(&mut Reader) -> V
) -> wgpu::Buffer {
    let mut vertices = Vec::<V>::new();
    let total_vertices = reader.read_u32() as usize;
    for _ in 0..total_vertices {
        vertices.push(f(reader));
    }
    *vertices_len = total_vertices as u32;
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX
    })
}
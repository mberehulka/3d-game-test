use super::Reader;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexNUS {
    pub position: [f32;3],
    pub normal: [f32;3],
    pub uv: [f32;2],
    pub joints: [u32;4],
    pub weights: [f32;4]
}
impl VertexNUS {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Uint32x4, 4 => Float32x4]
    };
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexU {
    pub position: [f32;3],
    pub uv: [f32;2]
}
impl VertexU {
    pub const _LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2]
    };
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexNU {
    pub position: [f32;3],
    pub normal: [f32;3],
    pub uv: [f32;2]
}
impl VertexNU {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2]
    };
}

pub enum VertexType {
    U,
    NU,
    NUS
}
impl VertexType {
    pub fn read(reader: &mut Reader) -> Self {
        match reader.read_string().as_str() {
            "U" => Self::U,
            "NU" => Self::NU,
            "NUS" => Self::NUS,
            vt => panic!("Invalid vertex type: {}", vt)
        }
    }
}
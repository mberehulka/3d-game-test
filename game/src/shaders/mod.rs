pub mod basic_anim;
pub mod terrain;

pub enum Material {
    BasicAnim(basic_anim::Material),
    Terrain(terrain::Material)
}

pub struct Shaders {
    pub basic_anim: basic_anim::Shader,
    pub terrain: terrain::Shader
}
impl Shaders {
    pub fn new(device: &wgpu::Device, surface_texture_format: wgpu::TextureFormat) -> Self {
        Self {
            basic_anim: basic_anim::Shader::new(device, surface_texture_format),
            terrain: terrain::Shader::new(device, surface_texture_format)
        }
    }
}
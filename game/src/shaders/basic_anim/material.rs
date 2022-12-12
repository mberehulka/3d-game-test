use std::sync::Arc;

use crate::assets::Texture;

pub struct Material {
    pub bind_group: Arc<wgpu::BindGroup>
}
impl Material {
    pub fn new(texture: Arc<Texture>) -> crate::shaders::Material {
        crate::shaders::Material::BasicAnim(Self {
            bind_group: texture.bind_group.clone()
        })
    }
}
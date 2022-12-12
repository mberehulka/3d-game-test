use std::sync::Arc;
use crate::assets::Texture;

pub struct Material {
    pub texture: Arc<Texture>
}
impl Material {
    pub fn new(texture: Arc<Texture>) -> crate::shaders::Material {
        crate::shaders::Material::Terrain(Self {
            texture
        })
    }
}
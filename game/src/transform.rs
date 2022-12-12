use cgmath::Matrix4;
use crate::{utils::{rotation_to_mat4, scale_to_mat4}, assets::Reader};

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: [f32;3],
    pub rotation: [f32;3],
    pub scale: [f32;3]
}
impl Transform {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            translation: reader.read_vec3(),
            rotation: reader.read_vec3(),
            scale: reader.read_vec3()
        }
    }
    pub fn mat(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.translation.into()) *
        rotation_to_mat4(self.rotation) *
        scale_to_mat4(self.scale)
    }
}
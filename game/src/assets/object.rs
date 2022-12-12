use std::sync::{Arc, Mutex};

use crate::{shaders::{Material, basic_anim}, context::Context, camera::Camera};

use super::{Mesh, Instances, Armature, Animation};

pub struct Object {
    pub mesh: Arc<Mesh>,
    pub instances: Instances,
    pub material: Material,
    pub armature: Option<Armature>
}
impl Object {
    pub fn set_animation(&self, animation: Arc<Animation>) {
        self.armature.as_ref().expect("Object has no armature").set_animation(animation)
    }
    pub fn update(&self, queue: &wgpu::Queue) {
        self.instances.update(&queue);
        if let Some(armature) = self.armature.as_ref() {
            armature.update(queue)
        }
    }
}

pub struct Objects(pub Mutex<Vec<Arc<Object>>>);
impl Objects {
    pub fn new() -> Self {
        Self(
            Mutex::new(Vec::new())
        )
    }
    pub fn add(
        &self,
        device: &wgpu::Device,
        mesh: Arc<Mesh>,
        material: Material,
        maximum_instances: usize
    ) -> Arc<Object> {
        let joints_len = mesh.joints.len();
        let object = Arc::new(Object {
            mesh: mesh.clone(),
            material,
            instances: Instances::new(device, maximum_instances),
            armature: if joints_len > 0 {
                Some(Armature::new(device, mesh))
            }else {
                None
            }
        });
        self.0.lock().unwrap().push(object.clone());
        object
    }
    pub fn draw<'r, 's: 'r>(
        render_pass: &mut wgpu::RenderPass<'r>,
        c: &'s Context,
        objects: &'s Vec<Arc<Object>>,
        camera: &'s Camera,
        basic_anim: &'s basic_anim::Shader
    ) {
        for object in objects.iter() {
            object.update(&c.queue);
            render_pass.set_bind_group(0, &camera.bind_group, &[]);
            render_pass.set_vertex_buffer(0, object.mesh.vertices_buffer.slice(..));
            render_pass.set_vertex_buffer(1, object.instances.buffer.slice(..));
            match &object.material {
                Material::BasicAnim(material) => {
                    render_pass.set_pipeline(&basic_anim.render_pipeline);
                    render_pass.set_bind_group(1, &material.bind_group, &[]);
                    render_pass.set_bind_group(2, &object.armature.as_ref().unwrap().bind_group, &[]);
                    render_pass.set_bind_group(3, &c.lights.sun.bind_group, &[]);
                    render_pass.draw(0..object.mesh.vertices_len, 0..object.instances.get_buffer_len());
                },
                Material::Terrain(material) => {
                    render_pass.set_pipeline(&c.shaders.terrain.render_pipeline);
                    render_pass.set_bind_group(1, &material.texture.bind_group, &[]);
                    render_pass.set_bind_group(2, &c.lights.sun.bind_group, &[]);
                    render_pass.draw(0..object.mesh.vertices_len, 0..1);
                }
            }
        }
    }
}
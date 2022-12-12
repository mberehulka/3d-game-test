use std::{sync::{Arc, Mutex}, path::Path, time::Instant};

use crate::{context::Context, assets::Reader};

use super::{Mesh, Texture, Animation};

pub struct Assets {
    pub meshes: Mutex<Vec<Arc<Mesh>>>,
    pub textures: Mutex<Vec<Arc<Texture>>>,
    pub animations: Mutex<Vec<Arc<Animation>>>
}
impl Assets {
    pub fn new() -> Self {
        Self {
            meshes: Mutex::new(Vec::new()),
            textures: Mutex::new(Vec::new()),
            animations: Mutex::new(Vec::new())
        }
    }
    pub fn load(&self, c: &Context, path: impl AsRef<Path>) {
        let start = Instant::now();
        let mut meshes_loaded = Vec::new();
        let mut textures_loaded = Vec::new();
        let mut animations_loaded = Vec::new();

        let mut meshes = self.meshes.lock().unwrap();
        let mut textures = self.textures.lock().unwrap();
        let mut animations = self.animations.lock().unwrap();

        let mut reader = Reader::new(path);
        while !reader.finished() {
            match reader.read_u8() {
                b'I' => {
                    let texture = Texture::load(&c.device, &c.queue, &mut reader);
                    textures_loaded.push(texture.name.clone());
                    textures.push(Arc::new(texture));
                },
                b'M' => {
                    let mesh = Mesh::load(&c.device, &mut reader);
                    meshes_loaded.push(mesh.name.clone());
                    meshes.push(Arc::new(mesh));
                },
                b'A' => {
                    let anim = Animation::load(&mut reader);
                    animations_loaded.push(anim.name.clone());
                    animations.push(Arc::new(anim));
                },
                asset_type => panic!("Invalid asset type: {}", asset_type)
            }
        }

        log::info!("Assets loaded: \n\tmeshes: {:?}, \n\ttextures: {:?}, \n\tanimations: {:?} \n\ttime: {:.2} sec",
            meshes_loaded,
            textures_loaded,
            animations_loaded,
            (Instant::now() - start).as_secs_f32());
    }
    pub fn get_mesh(&self, name: impl AsRef<str>) -> Arc<Mesh> {
        let name = name.as_ref().to_string();
        for v in self.meshes.lock().unwrap().iter() {
            if v.name == name {
                return v.clone()
            }
        }
        panic!("Mesh \"{name}\" not found")
    }
    pub fn get_animation(&self, name: impl AsRef<str>) -> Arc<Animation> {
        let name = name.as_ref().to_string();
        for v in self.animations.lock().unwrap().iter() {
            if v.name == name {
                return v.clone()
            }
        }
        panic!("Animation \"{name}\" not found")
    }
    pub fn get_texture(&self, name: impl AsRef<str>) -> Arc<Texture> {
        let name = name.as_ref().to_string();
        for v in self.textures.lock().unwrap().iter() {
            if v.name == name {
                return v.clone()
            }
        }
        panic!("Texture \"{name}\" not found")
    }
}
use crate::context::Context;

pub mod directional;

pub struct Lights {
    pub sun: directional::DirectionalLight
}

impl Lights {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            sun: directional::DirectionalLight::new(device, [45.,90.,0.], 2048)
        }
    }
    pub fn update(&self, queue: &wgpu::Queue) {
        self.sun.update(queue)
    }
    pub fn draw(&self, c: &Context) {
        self.sun.draw(c)
    }
}
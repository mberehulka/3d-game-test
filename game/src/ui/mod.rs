use std::sync::{Mutex, Arc};

mod square;  pub use square::*;

use crate::{assets::Texture, context::Context};

pub struct UI {
    pub depth_texture: Texture,
    pub square_shader: wgpu::RenderPipeline,
    pub squares: Mutex<Vec<Arc<Square>>>
}
impl UI {
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            depth_texture: Texture::depth("UI depth texture", device, surface_config.width, surface_config.height),
            square_shader: square::shader::new(device, surface_config.format),
            squares: Mutex::new(Vec::new())
        }
    }
    pub fn draw<'r, 's: 'r>(
        render_pass: &mut wgpu::RenderPass<'r>,
        c: &'s Context,
        squares: &'s Vec<Arc<Square>>
    ) {
        {
            render_pass.set_pipeline(&c.ui.square_shader);
            for square in squares.iter() {
                render_pass.set_vertex_buffer(0, square.buffer.slice(..));
                let texture = match &square.texture {
                    UIElementTexture::Owned(texture) => texture,
                    UIElementTexture::SunDepthBuffer => &c.lights.sun.shadow_texture
                };
                render_pass.set_bind_group(0, &texture.bind_group, &[]);
                render_pass.draw(0..6, 0..1);
            }
        }
    }
}

pub enum UIElementTexture {
    Owned(Arc<Texture>),
    SunDepthBuffer
}
impl Into<UIElementTexture> for Arc<Texture> {
    fn into(self) -> UIElementTexture {
        UIElementTexture::Owned(self)
    }
}
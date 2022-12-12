use std::sync::Mutex;

pub mod basic_anim;
pub mod terrain;

pub enum Material {
    BasicAnim(basic_anim::Material),
    Terrain(terrain::Material)
}

pub struct Shaders {
    pub a: Mutex<f32>,
    pub b: Mutex<f32>,
    pub c: Mutex<f32>,
    pub basic_anim: Mutex<basic_anim::Shader>,
    pub terrain: terrain::Shader
}
impl Shaders {
    pub fn new(device: &wgpu::Device, surface_texture_format: wgpu::TextureFormat) -> Self {
        let a = 0.03;
        let b = 0.;
        let c = 0.0075;
        Self {
            a: Mutex::new(a),
            b: Mutex::new(b),
            c: Mutex::new(c),
            basic_anim: Mutex::new(basic_anim::Shader::new(device, surface_texture_format, a, b, c)),
            terrain: terrain::Shader::new(device, surface_texture_format)
        }
    }
    pub fn increase_a(&self, device: &wgpu::Device, surface_texture_format: wgpu::TextureFormat, v: f32) {
        let mut a = self.a.lock().unwrap();
        let b = self.b.lock().unwrap();
        let c = self.c.lock().unwrap();
        *a += v;
        *self.basic_anim.lock().unwrap() = basic_anim::Shader::new(device, surface_texture_format, *a, *b, *c);
        println!("a: {}, b: {}, c: {}", *a, &b, *c);
    }
    pub fn increase_b(&self, device: &wgpu::Device, surface_texture_format: wgpu::TextureFormat, v: f32) {
        let a = self.a.lock().unwrap();
        let mut b = self.b.lock().unwrap();
        let c = self.c.lock().unwrap();
        *b += v;
        *self.basic_anim.lock().unwrap() = basic_anim::Shader::new(device, surface_texture_format, *a, *b, *c);
        println!("a: {}, b: {}, c: {}", *a, &b, *c);
    }
    pub fn increase_c(&self, device: &wgpu::Device, surface_texture_format: wgpu::TextureFormat, v: f32) {
        let a = self.a.lock().unwrap();
        let b = self.b.lock().unwrap();
        let mut c = self.c.lock().unwrap();
        *c += v;
        *self.basic_anim.lock().unwrap() = basic_anim::Shader::new(device, surface_texture_format, *a, *b, *c);
        println!("a: {:.9}, b: {:.9}, c: {:.9}", *a, &b, *c);
    }
}
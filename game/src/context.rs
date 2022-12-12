use std::{sync::{Mutex, Arc}, path::Path};
use wgpu::{Surface, Device, Queue, SurfaceConfiguration};
use winit::{event_loop::EventLoop, window::Window, dpi::PhysicalSize};
use crate::{settings::Settings, utils, camera::Camera, window, shaders::{Shaders, Material},
    assets::{Object, Mesh, Texture, Objects, Assets}, cursor::Cursor,
    ui::{UI, Square, UIElementTexture}, light::Lights};

pub struct Context {
    pub window: Window,
    pub settings: Settings,
    pub surface: Surface,
    pub surface_config: Mutex<SurfaceConfiguration>,
    pub device: Device,
    pub queue: Queue,
    pub camera: Mutex<Camera>,
    pub depth_texture: Mutex<Texture>,
    pub shaders: Shaders,
    pub objects: Objects,
    pub cursor: Cursor,
    pub ui: UI,
    pub lights: Lights,
    pub assets: Assets,
    pub character: Mutex<Option<Arc<Object>>>
}

impl Context {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        crate::logger::start();
        let settings = Settings::read();

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let window = window::new(&settings, &event_loop);
        
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = utils::create_adapter(&instance, &surface);
        let (device, queue) = utils::create_device_queue(&adapter);
        
        let surface_config = utils::configure_surface(&settings, &window, &device, &adapter, &surface);

        let cursor = Cursor::new(&window);
        let camera = Camera::new(&settings, &device, &window, &cursor);
        let depth_texture = Texture::depth("Main depth texture", &device, surface_config.width, surface_config.height);
    
        let shaders = Shaders::new(&device, surface_config.format);

        let ui = UI::new(&device, &surface_config);
        let lights = Lights::new(&device);

        Self {
            window, settings, surface, device, queue, cursor, shaders, ui, lights,
            camera: Mutex::new(camera),
            depth_texture: Mutex::new(depth_texture),
            surface_config: Mutex::new(surface_config),
            objects: Objects::new(),
            character: Mutex::new(None),
            assets: Assets::new()
        }
    }
    pub fn resize(&self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return }
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = new_size.width;
        surface_config.height = new_size.height;
        self.surface.configure(&self.device, &surface_config);
        self.camera.lock().unwrap().resize(&self.settings, new_size);
        *self.depth_texture.lock().unwrap() = Texture::depth("Main depth texture", &self.device, surface_config.width, surface_config.height);
    }
    pub fn add_object(
        &self,
        mesh: Arc<Mesh>,
        material: Material,
        maximum_instances: usize
    ) -> Arc<Object> {
        self.objects.add(&self.device, mesh, material, maximum_instances)
    }
    pub fn add_square(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [[f32;4];4],
        texture: impl Into<UIElementTexture>
    ) -> Arc<Square> {
        let square = Square::new(&self.device, texture, x, y, width, height, color);
        self.ui.squares.lock().unwrap().push(square.clone());
        square
    }
    pub fn load_assets(&self, path: impl AsRef<Path>) {
        self.assets.load(self, path)
    }
}
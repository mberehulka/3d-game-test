use std::sync::Mutex;
use wgpu::{Surface, Device, Queue, SurfaceConfiguration};
use winit::{event_loop::EventLoop, window::Window, dpi::PhysicalSize};

use crate::{settings::Settings, utils, camera::Camera, depth_texture::DepthTexture, window};

pub struct Context {
    pub window: Window,
    pub settings: Settings,
    pub surface: Surface,
    pub surface_config: Mutex<SurfaceConfiguration>,
    pub device: Device,
    pub queue: Queue,
    pub camera: Mutex<Camera>,
    pub depth_texture: Mutex<DepthTexture>
}

impl Context {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        crate::logger::start();
        let settings = Settings::read();

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let window = window::new(&settings, &event_loop);
        
        let surface = unsafe { instance.create_surface(&window) };
        trace!("Creating adapter");
        let adapter = utils::create_adapter(&instance, &surface);
        trace!("Creating device");
        let (device, queue) = utils::create_device_queue(&adapter);
        
        trace!("Configuring surface");
        let surface_config = utils::configure_surface(&settings, &window, &device, &adapter, &surface);
        
        let camera = Camera::new(&device, &queue, &window, [0.,1.,3.], [3.1415,0.]);
        let depth_texture = DepthTexture::new(&device, &surface_config);
    
        Self {
            window, settings, surface, device, queue,
            camera: Mutex::new(camera),
            depth_texture: Mutex::new(depth_texture),
            surface_config: Mutex::new(surface_config)
        }
    }
    pub fn resize(&self, new_size: PhysicalSize<u32>) {
        trace!("Resizing");
        if new_size.width == 0 || new_size.height == 0 { return }
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = new_size.width;
        surface_config.height = new_size.height;
        self.surface.configure(&self.device, &surface_config);
        self.camera.lock().unwrap().resize(&new_size);
        *self.depth_texture.lock().unwrap() = DepthTexture::new(&self.device, &surface_config);
    }
}
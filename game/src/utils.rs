use futures::executor::block_on;
use winit::window::Window;

use crate::settings::Settings;

pub fn create_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false
    })).unwrap();
    instance.enumerate_adapters(wgpu::Backends::all()).next().unwrap()
}

pub fn create_device_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None
        },
        None
    )).unwrap()
}

pub fn configure_surface(
    settings: &Settings,
    window: &Window,
    device: &wgpu::Device,
    adapter: &wgpu::Adapter,
    surface: &wgpu::Surface
) -> wgpu::SurfaceConfiguration {
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(adapter)[0],
        width: size.width,
        height: size.height,
        present_mode: if settings.vsync {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::Immediate
        },
        alpha_mode: wgpu::CompositeAlphaMode::Opaque
    };
    surface.configure(device, &config);
    config
}
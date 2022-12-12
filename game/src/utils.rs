use std::ops::{SubAssign, Sub, Mul, AddAssign};
use cgmath::{Vector3, Point3, Matrix4, Rad, Quaternion, Rotation3, Matrix3};
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

#[derive(Copy, Clone)]
pub struct SmoothValue<T> {
    value: T,
    target: T,
    smoothness: T,
    speed: T
}
impl<T: Copy + SubAssign + AddAssign + Sub<Output = T> + Mul<Output = T>> SmoothValue<T> {
    pub fn new(value: T, speed: T, smoothness: T) -> Self {
        Self {
            value, speed, smoothness,
            target: value
        }
    }
    pub fn get(&mut self) -> T {
        self.target -= (self.target - self.value) * self.smoothness;
        self.target
    }
    pub fn change(&mut self, value: T) {
        self.value += value * self.speed
    }
}

pub struct SmoothValueBounded<T> {
    value: T,
    target: T,
    speed: T,
    smoothness: T,
    min: T,
    max: T
}
impl<T: Copy + SubAssign + AddAssign + PartialOrd + Sub<Output = T> + Mul<Output = T>> SmoothValueBounded<T> {
    pub fn new(value: T, speed: T, smoothness: T, min: T, max: T) -> Self {
        Self {
            value, speed, min, max, smoothness,
            target: value
        }
    }
    pub fn get(&mut self) -> T {
        self.target -= (self.target - self.value) * self.smoothness;
        self.target
    }
    pub fn change(&mut self, value: T) {
        self.value += value * self.speed;
        if self.value > self.max { self.value = self.max }
        if self.value < self.min { self.value = self.min }
    }
}

#[inline]
pub fn rotation_to_mat4(v: [f32;3]) -> Matrix4<f32> {
    Matrix4::from_angle_z(Rad(v[2])) *
    Matrix4::from_angle_y(Rad(v[1])) *
    Matrix4::from_angle_x(Rad(v[0]))
}
#[inline]
pub fn scale_to_mat4(v: [f32;3]) -> Matrix4<f32> {
    Matrix4::from_nonuniform_scale(v[0], v[1], v[2])
}
#[inline]
pub fn rotation_to_quaternion(v: [f32;3]) -> Quaternion<f32> {
    Quaternion::from_angle_z(Rad(v[2])) *
    Quaternion::from_angle_y(Rad(v[1])) *
    Quaternion::from_angle_x(Rad(v[0]))
}
#[inline]
pub fn vec3_to_point3(v: Vector3<f32>) -> Point3<f32> {
    Point3 { x: v.x, y: v.y, z: v.z }
}
#[inline]
pub fn vec3_linear_interpolation(a: [f32;3], b: [f32;3], t: f32) -> [f32;3] {
    [
        a[0] + ((b[0] - a[0]) * t),
        a[1] + ((b[1] - a[1]) * t),
        a[2] + ((b[2] - a[2]) * t)
    ]
}
#[inline]
pub fn mat4_to_mat3(v: Matrix4<f32>) -> Matrix3<f32> {
    Matrix3 {
        x: Vector3 { x: v.x.x, y: v.x.y, z: v.x.z },
        y: Vector3 { x: v.y.x, y: v.y.y, z: v.y.z },
        z: Vector3 { x: v.z.x, y: v.z.y, z: v.z.z }
    }
}
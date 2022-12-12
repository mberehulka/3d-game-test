use std::sync::{atomic::{AtomicU32, AtomicBool}, Mutex};

use wgpu::util::DeviceExt;

#[repr(C, align(8))]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceTransform {
    pub position: [f32;3],
    pub scale: [f32;3]
}

pub struct Instances {
    pub buffer: wgpu::Buffer,
    transforms: Mutex<Vec<InstanceTransform>>,
    buffer_len: AtomicU32,
    needs_update: AtomicBool
}
#[allow(dead_code)]
impl Instances {
    pub fn new(device: &wgpu::Device, maximum: usize) -> Self {
        let mut transforms = Vec::with_capacity(maximum as usize);
        for _ in 0..maximum {
            transforms.push(InstanceTransform {
                position: [0.;3],
                scale: [0.;3]
            });
        }
        Self {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&transforms),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }),
            transforms: Mutex::new(transforms),
            buffer_len: AtomicU32::new(0),
            needs_update: AtomicBool::new(false)
        }
    }
    pub fn get_buffer_len(&self) -> u32 {
        self.buffer_len.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn set_buffer_len(&self, v: u32) {
        self.buffer_len.store(v, std::sync::atomic::Ordering::SeqCst)
    }
    pub fn clear(&self) {
        self.set_buffer_len(0)
    }
    pub fn add(&self, transform: InstanceTransform) {
        let i = self.get_buffer_len();
        let mut transforms = self.transforms.lock().unwrap();
        if i as usize == transforms.len() {
            return log::warn!("Can not add any more instances");
        }
        transforms[i as usize] = transform;
        self.buffer_len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.needs_update.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn update(&self, queue: &wgpu::Queue) {
        if self.needs_update.load(std::sync::atomic::Ordering::SeqCst) {
            let transforms = self.transforms.lock().unwrap();
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&transforms));
            self.needs_update.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
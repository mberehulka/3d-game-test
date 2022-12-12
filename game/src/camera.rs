use std::sync::Arc;

use cgmath::{Matrix4, Quaternion, Rotation3, Vector3, Rad};
use wgpu::{util::DeviceExt, Queue};
use winit::{window::Window, dpi::PhysicalSize};

use crate::{settings::Settings, utils::{vec3_to_point3, SmoothValue, SmoothValueBounded}, cursor::Cursor, assets::Object};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraBinding {
    projection: [[f32;4];4],
    position: [f32;4]
}

pub enum CameraTarget {
    Joint {
        object: Arc<Object>,
        offset: Vector3<f32>,
        scale: f32,
        joint_id: usize
    },
    Point(Vector3<f32>)
}

pub struct CameraValues {
    perspective: Matrix4<f32>,
    position: Vector3<f32>,
    rotation_x: SmoothValueBounded<f32>,
    rotation_y: SmoothValue<f32>,
    distance: SmoothValueBounded<f32>,
    target: CameraTarget,
    center: Vector3<f32>
}
impl CameraValues {
    pub fn new(
        settings: &Settings,
        window: &Window
    ) -> Self {
        let target = Vector3::new(0., 1.5, 0.);
        let window_size = window.inner_size();
        let aspect = window_size.width as f32 / window_size.height as f32;
        let perspective = cgmath::perspective(cgmath::Deg(settings.fov), aspect, settings.near, settings.far);
        Self {
            perspective,
            position: Vector3::new(0., 0., 0.),
            rotation_x: SmoothValueBounded::new(-0.78539816, 0.0015, 0.12, -1.3, 0.),
            rotation_y: SmoothValue::new(0., 0.0015, 0.12),
            distance: SmoothValueBounded::new(4., 0.5, 0.1, 1., 10.),
            target: CameraTarget::Point(target),
            center: target
        }
    }
    pub fn update(&mut self, cursor: &Cursor) {
        self.distance.change(-cursor.wheel_movement());
        let movement = cursor.get_movement();
        self.rotation_x.change(movement.y as f32);
        self.rotation_y.change(movement.x as f32);
        let rotation = Quaternion::from_angle_y(Rad(self.rotation_y.get())) * Quaternion::from_angle_x(Rad(self.rotation_x.get()));
        self.position = rotation * Vector3::new(0., 0., self.distance.get());
        let point = match &self.target {
            CameraTarget::Joint { object, offset, scale, joint_id } =>
                (object.armature.as_ref().unwrap().get_joint_translation(*joint_id) * *scale) + offset,
            CameraTarget::Point(point) => *point
        };
        self.position += point;
        self.center = point;
    }
    pub fn get_projection(&self) -> Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(
            vec3_to_point3(self.position),
            vec3_to_point3(self.center),
            [0., 1., 0.].into()
        );
        (self.perspective * view).into()
    }
    pub fn resize(&mut self, settings: &Settings, new_size: PhysicalSize<u32>) {
        let aspect = new_size.width as f32 / new_size.height as f32;
        self.perspective = cgmath::perspective(cgmath::Deg(settings.fov), aspect, settings.near, settings.far);
    }
    pub fn get_position(&self) -> [f32;4] {
        [self.position.x, self.position.y, self.position.z, 1.]
    }
}

pub struct Camera {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub values: CameraValues
}

impl Camera {
    pub fn new(
        settings: &Settings,
        device: &wgpu::Device,
        window: &Window,
        cursor: &Cursor
    ) -> Self {
        let mut values = CameraValues::new(settings, window);
        values.update(cursor);
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[CameraBinding {
                    projection: values.get_projection().into(),
                    position: values.get_position()
                }]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ]
        });
        Self {
            buffer,
            bind_group,
            values
        }
    }
    pub fn update(&mut self, queue: &Queue, cursor: &Cursor) {
        self.values.update(cursor);
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[CameraBinding {
            projection: self.values.get_projection().into(),
            position: self.values.get_position()
        }]))
    }
    pub fn resize(&mut self, settings: &Settings, new_size: PhysicalSize<u32>) {
        self.values.resize(settings, new_size)
    }
    pub fn set_target(&mut self, target: CameraTarget) {
        self.values.target = target
    }
}

pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        ]
    })
}
use std::sync::{Arc, Mutex};
use cgmath::{Vector3, Matrix4};
use wgpu::util::DeviceExt;

use crate::utils::{vec3_linear_interpolation, scale_to_mat4, rotation_to_quaternion, mat4_to_mat3};
use super::{Mesh, Animation};

pub const MAX_JOINTS: usize = 128;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArmaturePosesBinding {
    pub mats: [[[f32;4];4];MAX_JOINTS]
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArmatureRotationsBinding {
    pub mats: [[[f32;4];4];MAX_JOINTS]
}
pub struct Armature {
    pub bind_group: wgpu::BindGroup,
    poses_binding: Mutex<ArmaturePosesBinding>,
    rotations_binding: Mutex<ArmatureRotationsBinding>,
    poses_buffer: wgpu::Buffer,
    rotations_buffer: wgpu::Buffer,
    mesh: Arc<Mesh>,
    animation: Mutex<Option<Arc<Animation>>>,
    speed: Mutex<f32>,
    time: Mutex<f32>
}
impl Armature {
    pub fn new(device: &wgpu::Device, mesh: Arc<Mesh>) -> Self {
        let poses_binding = ArmaturePosesBinding {
            mats: [[[0.;4];4];MAX_JOINTS]
        };
        let rotations_binding = ArmatureRotationsBinding {
            mats: [[[0.;4];4];MAX_JOINTS]
        };
        let poses_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[poses_binding]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        let rotations_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[rotations_binding]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &armature_bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: poses_buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: rotations_buffer.as_entire_binding()
                }
            ]
        });
        Self {
            poses_buffer,
            rotations_buffer,
            bind_group,
            poses_binding: Mutex::new(poses_binding),
            rotations_binding: Mutex::new(rotations_binding),
            mesh,
            animation: Mutex::new(None),
            speed: Mutex::new(0.1),
            time: Mutex::new(0.)
        }
    }
    #[inline]
    pub fn frames(&self) -> usize {
        if let Some(animation) = self.animation.lock().unwrap().as_ref() {
            animation.frames.len()
        } else { 0 }
    }
    pub fn set_animation(&self, animation: Arc<Animation>) {
        if self.mesh.joints.len() != animation.joints_length {
            panic!("Animation has {} joints, but mesh has {} joints", animation.joints_length, self.mesh.joints.len())
        }
        *self.animation.lock().unwrap() = Some(animation);
        *self.time.lock().unwrap() = 0.;
    }
    pub fn update(&self, queue: &wgpu::Queue) {
        if self.animation.lock().unwrap().as_ref().is_some() {
            let mut poses_binding = self.poses_binding.lock().unwrap();
            let mut rotations_binding = self.rotations_binding.lock().unwrap();

            for (joint_id, joint) in self.mesh.joints.iter().enumerate() {
                let mat = self.get_joint_pose(joint_id) * joint.ibm;
                poses_binding.mats[joint_id] = mat.into();
                rotations_binding.mats[joint_id] = Matrix4::from(mat4_to_mat3(mat)).into();
            }

            queue.write_buffer(&self.poses_buffer, 0, bytemuck::cast_slice(&[*poses_binding]));
            queue.write_buffer(&self.rotations_buffer, 0, bytemuck::cast_slice(&[*rotations_binding]));

            let mut time = self.time.lock().unwrap();
            *time += *self.speed.lock().unwrap();
            if *time >= self.frames() as f32 {
                *time = 0.;
            }
        }
    }
    pub fn get_joint_pose(&self, joint_id: usize) -> Matrix4<f32> {
        if let Some(animation) = self.animation.lock().unwrap().as_ref() {
            let time = *self.time.lock().unwrap();
            let current_frame = time as usize;
            let last_frame = animation.frames.len() - 1;
            let current_joint = &animation.frames[current_frame].joints[joint_id].pose;
            if current_frame == last_frame {
                current_joint.mat()
            } else {
                let next_joint = &animation.frames[current_frame + 1].joints[joint_id].pose;
                let t = time - current_frame as f32;
                Matrix4::from_translation(vec3_linear_interpolation(current_joint.translation, next_joint.translation, t).into()) *
                Matrix4::from(rotation_to_quaternion(current_joint.rotation).nlerp(rotation_to_quaternion(next_joint.rotation), t)) *
                scale_to_mat4(vec3_linear_interpolation(current_joint.scale, next_joint.scale, t))
            }
        } else { self.mesh.joints[joint_id].tpose }
    }
    pub fn get_joint_translation(&self, joint_id: usize) -> Vector3<f32> {
        if let Some(animation) = self.animation.lock().unwrap().as_ref() {
            let time = *self.time.lock().unwrap();
            let current_frame = time as usize;
            let last_frame = animation.frames.len() - 1;
            let current_joint = &animation.frames[current_frame].joints[joint_id].pose;
            if current_frame == last_frame {
                current_joint.translation.into()
            } else {
                let next_joint = &animation.frames[current_frame + 1].joints[joint_id].pose;
                let t = time - current_frame as f32;
                vec3_linear_interpolation(current_joint.translation, next_joint.translation, t).into()
            }
        } else { Vector3 { x: 0., y: 0., z: 0. } }
    }
}

pub fn armature_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX,
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
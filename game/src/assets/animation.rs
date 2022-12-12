use crate::{assets::Reader, transform::Transform};

pub struct AnimationJoint {
    pub pose: Transform
}

pub struct AnimationFrame {
    pub joints: Vec<AnimationJoint>
}

pub struct Animation {
    pub name: String,
    pub joints_length: usize,
    pub frames: Vec<AnimationFrame>
}
impl Animation {
    pub fn load(reader: &mut Reader) -> Self {
        let name = reader.read_string();

        let joints_length = reader.read_u8() as usize;
        let frames_length = reader.read_u32() as usize;

        let mut frames = Vec::with_capacity(frames_length);
        for _ in 0..frames_length {
            let mut frame = AnimationFrame {
                joints: Vec::with_capacity(joints_length)
            };
            for _ in 0..joints_length {
                frame.joints.push(AnimationJoint {
                    pose: Transform::read(reader)
                })
            }
            frames.push(frame)
        }

        reader.read_end(&name);

        Self {
            name, frames, joints_length
        }
    }
}
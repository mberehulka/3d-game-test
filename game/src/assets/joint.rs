use cgmath::Matrix4;
use super::{Reader, MAX_JOINTS};

pub struct Joint {
    pub name: String,
    pub id: usize,
    pub tpose: Matrix4<f32>,
    pub tpose_local: Matrix4<f32>,
    pub ibm: Matrix4<f32>,
    /// nearest parent
    pub parent: usize,
    /// from farthest to nearest
    pub parents: Vec<usize>
}
impl Joint {
    #[inline]
    pub fn read(reader: &mut Reader) -> Vec<Joint> {
        let joints_length = reader.read_u8() as usize;
        if joints_length >= MAX_JOINTS {
            panic!("Skeleton can not have more than {} joints", MAX_JOINTS)
        }
        let mut joints = Vec::with_capacity(joints_length);
        let mut joint_id = 0;
        while joint_id < joints_length {
            joints.push(Joint {
                name: reader.read_string(),
                id: joint_id,
                parent: reader.read_u8() as usize,
                parents: {
                    let mut parents = Vec::new();
                    let mut parent = reader.read_u8() as usize;
                    while parent != 255 {
                        parents.push(parent);
                        parent = reader.read_u8() as usize;
                    }
                    parents
                },
                tpose: reader.read_mat4x4().into(),
                tpose_local: reader.read_mat4x4().into(),
                ibm: reader.read_mat4x4().into()
            });
            joint_id += 1;
        }
        joints
    }
}
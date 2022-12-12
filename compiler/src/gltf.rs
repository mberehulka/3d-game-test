use std::{path::Path, time::Instant};

use cgmath::{Matrix4, SquareMatrix};

use crate::{writer::Writer, config::{VertexType, Config}};

pub fn file(path: impl AsRef<Path>) -> Vec<u8> {
    let start = Instant::now();
    let path = path.as_ref();
    let mut res = Writer(Vec::new());
    res.0.push(b'M');
    res.append_string(path.with_extension("").file_name().unwrap().to_string_lossy().to_string());

    let conf = Config::new(path.parent().unwrap());

    let (gltf, buffers, _) = match gltf::import(path) { Ok(v)=>v, Err(e) => panic!("{}, {:?}", e, path.display()) };
    match conf.vertex_type {
        VertexType::Basic => write_vertices(&mut res, &conf, &gltf, &buffers, |res, indices, ps, _, _, _, _| {
            let mut i = 0;
            let l = indices.len();
            while i < l {
                let idx = indices[i] as usize;
                res.append_vec3_f32(ps[idx]);
                i += 1;
            }
        }),
        VertexType::U => write_vertices(&mut res, &conf, &gltf, &buffers, |res, indices, ps, _, uvs, _, _| {
            let uvs = uvs.unwrap();
            let mut i = 0;
            let l = indices.len();
            while i < l {
                let idx = indices[i] as usize;
                res.append_vec3_f32(ps[idx]);
                res.append_vec2_f32(uvs[idx]);
                i += 1;
            }
        }),
        VertexType::NU => write_vertices(&mut res, &conf, &gltf, &buffers, |res, indices, ps, ns, uvs, _, _| {
            let ns = ns.unwrap();
            let uvs = uvs.unwrap();
            let mut i = 0;
            let l = indices.len();
            while i < l {
                let idx = indices[i] as usize;
                res.append_vec3_f32(ps[idx]);
                res.append_vec3_f32(ns[idx]);
                res.append_vec2_f32(uvs[idx]);
                i += 1;
            }
        }),
        VertexType::NUS => write_vertices(&mut res, &conf, &gltf, &buffers, |res, indices, ps, ns, uvs, js, ws| {
            let ns = ns.unwrap();
            let uvs = uvs.unwrap();
            let js = js.unwrap();
            let ws = ws.unwrap();
            let mut i = 0;
            let l = indices.len();
            while i < l {
                let idx = indices[i] as usize;
                res.append_vec3_f32(ps[idx]);
                res.append_vec3_f32(ns[idx]);
                res.append_vec2_f32(uvs[idx]);
                res.append_joints(js[idx]);
                res.append_vec4_f32(ws[idx]);
                i += 1;
            }
        })
    }
    match conf.vertex_type {
        VertexType::NUS => {
            let skin = gltf.skins().next().unwrap();
            let reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
            let ibms: Vec<[[f32; 4]; 4]> = reader.read_inverse_bind_matrices().unwrap().collect();
            let joints: Vec<gltf::Node> = skin.joints().collect();

            let mut joints_poses = Vec::with_capacity(joints.len());
            for joint_id in 0..joints.len() {
                joints_poses.push(Matrix4::from(ibms[joint_id]).invert().unwrap());
            }
            let mut joints_poses_local = joints_poses.clone();
            let mut joints_parents = Vec::with_capacity(joints.len());
            for (joint_id, joint) in joints.iter().enumerate() {
                let mut parent = get_gltf_node_parent_id(&joints, &joint);
                if parent != 255 {
                    joints_poses_local[joint_id] = joints_poses[joint_id] * Matrix4::from(ibms[parent as usize])
                }
                let mut parents = Vec::new();
                loop {
                    if parent == 255 { break }
                    parents.push(parent);
                    parent = get_gltf_node_parent_id(&joints, &joints[parent as usize]);
                }
                parents.reverse();
                joints_parents.push(parents);
            }

            res.0.push(joints.len()as u8);
            for (joint_id, joint) in joints.iter().enumerate() {
                res.append_string(joint.name().unwrap().to_string().replace('#', ""));
                res.0.push(get_gltf_node_parent_id(&joints, &joint));
                for parent in &joints_parents[joint_id] {
                    res.0.push(*parent);
                }
                res.0.push(255);
                res.append_mat4x4(joints_poses[joint_id].into());
                res.append_mat4x4(joints_poses_local[joint_id].into());
                res.append_mat4x4(ibms[joint_id]);
            }
        }
        VertexType::Basic | VertexType::U | VertexType::NU =>  {}
    }
    res.append_bytes(b"END");

    println!("gltf mesh: {}, compiled in: {:.2} sec", path.display(), (Instant::now() - start).as_secs_f64());
    res.0
}

#[inline]
fn write_vertices(
    res: &mut Writer,
    conf: &Config,
    gltf: &gltf::Document,
    buffers: &Vec<gltf::buffer::Data>,
    f: fn(&mut Writer, Vec<u32>, Vec<[f32;3]>, Option<Vec<[f32;3]>>, Option<Vec<[f32;2]>>, Option<Vec<[u16;4]>>, Option<Vec<[f32;4]>>)
) {
    let mut vertices = 0u32;
    match conf.vertex_type {
        VertexType::Basic => res.append_bytes(b"Basic#"),
        VertexType::NUS => res.append_bytes(b"NUS#"),
        VertexType::NU => res.append_bytes(b"NU#"),
        VertexType::U => res.append_bytes(b"U#")
    }
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
            vertices += indices.len() as u32;
        }
    }
    res.append_u32(vertices);
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions: Vec<[f32;3]> = reader.read_positions().unwrap().collect();
            let normals: Option<Vec<[f32;3]>> = match reader.read_normals() {
                Some(v) => Some(v.collect()), None => None };
            let uvs: Option<Vec<[f32;2]>> = match reader.read_tex_coords(0) {
                Some(v) => Some(v.into_f32().collect()), None => None };
            let joints: Option<Vec<[u16;4]>> = match reader.read_joints(0) {
                Some(v) => Some(v.into_u16().collect()), None => None };
            let weights: Option<Vec<[f32;4]>> = match reader.read_weights(0) {
                Some(v) => Some(v.into_f32().collect()), None => None };
            let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
            f(res, indices, positions, normals, uvs, joints, weights);
        }
    }
}

fn get_gltf_node_parent_id(joints: &Vec<gltf::Node>, j: &gltf::Node) -> u8 {
    for (parent_id, joint) in joints.iter().enumerate() {
        for child in joint.children() {
            if child.index() == j.index() {
                return parent_id as u8
            }
        }
    }
    255
}
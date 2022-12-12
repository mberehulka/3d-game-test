import bpy, struct, time, mathutils
from pathlib import Path
from mathutils import Matrix, Euler

res = bytearray()

def euler_mat(e: Euler):
    return mathutils.Matrix.Rotation(e.z, 4, 'Z') @ \
           mathutils.Matrix.Rotation(e.y, 4, 'Y') @ \
           mathutils.Matrix.Rotation(e.x, 4, 'X')

def write_u32(v: any):  res.extend(v.to_bytes(4, byteorder='big', signed=False))
def write_byte(v: any): res.append(v[0])
def write_bytes(v: any): res.extend(v)
def write_str(v: any):  res.extend(str.encode(str(v))+b'#')
def write_u8(v: any):
    if v > 255: raise Exception("Value is bigger than 255")
    res.extend(v.to_bytes(1, byteorder='big', signed=False))
def write_mat4x4(mat: any):
    res.extend(struct.pack(">f", mat[0][0])); res.extend(struct.pack(">f", mat[1][0]))
    res.extend(struct.pack(">f", mat[2][0])); res.extend(struct.pack(">f", mat[3][0]))
    res.extend(struct.pack(">f", mat[0][1])); res.extend(struct.pack(">f", mat[1][1]))
    res.extend(struct.pack(">f", mat[2][1])); res.extend(struct.pack(">f", mat[3][1]))
    res.extend(struct.pack(">f", mat[0][2])); res.extend(struct.pack(">f", mat[1][2]))
    res.extend(struct.pack(">f", mat[2][2])); res.extend(struct.pack(">f", mat[3][2]))
    res.extend(struct.pack(">f", mat[0][3])); res.extend(struct.pack(">f", mat[1][3]))
    res.extend(struct.pack(">f", mat[2][3])); res.extend(struct.pack(">f", mat[3][3]))
def write_vec3(v: any):
    res.extend(struct.pack(">f", v[0])); res.extend(struct.pack(">f", v[1])); res.extend(struct.pack(">f", v[2]))
def write_vec4(v: any):
    res.extend(struct.pack(">f", v[0])); res.extend(struct.pack(">f", v[1]))
    res.extend(struct.pack(">f", v[2])); res.extend(struct.pack(">f", v[3]))
def clear_scene(): bpy.ops.wm.read_factory_settings(use_empty=True)

def set_last_frame():
    if bpy.data.actions:
        action_list = [action.frame_range for action in bpy.data.actions]
        keys = (sorted(set([item for sublist in action_list for item in sublist])))
        bpy.context.scene.frame_end = int(keys[-1])
    else: raise Exception("No actions found")

def export_frames():
    frames = bpy.context.scene.frame_end
    write_u8(len(bpy.context.selected_pose_bones))
    write_u32(frames)
    for frame in range(frames):
        bpy.context.scene.frame_set(frame)
        for bone in bpy.context.selected_pose_bones:
            bpy.context.view_layer.update()
            write_vec3(bone.matrix.to_translation())
            write_vec3(bone.matrix.to_euler())
            write_vec3(bone.matrix.to_scale())
            # write_mat4x4(bone.matrix @ (bone.parent.matrix.inverted_safe()) if bone.parent else bone.matrix)

def export_animation(path: Path, start: time):
    write_byte(b'A')
    write_str(path.name.split('.')[0])
    bpy.ops.object.mode_set(mode='POSE')
    for obj in bpy.context.scene.objects: obj.select_set(True)
    set_last_frame()
    export_frames()
    write_bytes(b'END')
    print(f"animation: ./{str(path)}, compiled in : {(time.time() - start):.2f} sec")

for path in Path("./assets/animations/").glob("**/*.gltf"):
    start = time.time()
    clear_scene()
    bpy.ops.import_scene.gltf(filepath=str(path))
    export_animation(path, start)
for path in Path("./assets/animations/").glob("**/*.fbx"):
    start = time.time()
    clear_scene()
    bpy.ops.import_scene.fbx(filepath=str(path))
    export_animation(path, start)

open("./assets/compiled.bin", "ab").write(res)
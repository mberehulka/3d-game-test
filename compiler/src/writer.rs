pub struct Writer(pub Vec<u8>);
impl Writer {
    #[inline]
    pub fn append_string(&mut self, v: String) {
        self.0.append(&mut v.as_bytes().to_vec());
        self.0.push(b'#');
    }
    #[inline]
    pub fn append_bytes(&mut self, v: &[u8]) {
        self.0.append(&mut v.to_vec());
    }
    #[inline]
    pub fn append_u32(&mut self, v: u32) {
        let v = v.to_be_bytes();
        self.0.push(v[0]); self.0.push(v[1]); self.0.push(v[2]); self.0.push(v[3]);
    }
    #[inline]
    pub fn append_f32(&mut self, v: f32) {
        let v = v.to_be_bytes();
        self.0.push(v[0]); self.0.push(v[1]); self.0.push(v[2]); self.0.push(v[3]);
    }
    #[inline]
    pub fn append_vec3_f32(&mut self, v: [f32;3]) {
        self.append_f32(v[0]); self.append_f32(v[1]); self.append_f32(v[2])
    }
    #[inline]
    pub fn append_vec2_f32(&mut self, v: [f32;2]) {
        self.append_f32(v[0]); self.append_f32(v[1])
    }
    #[inline]
    pub fn append_vec4_f32(&mut self, v: [f32;4]) {
        self.append_f32(v[0]); self.append_f32(v[1]); self.append_f32(v[2]); self.append_f32(v[3])
    }
    #[inline]
    pub fn append_joints(&mut self, v: [u16;4]) {
        self.0.push(v[0]as u8); self.0.push(v[1]as u8); self.0.push(v[2]as u8); self.0.push(v[3]as u8);
    }
    #[inline]
    pub fn append_mat4x4(&mut self, v: [[f32;4];4]) {
        self.append_vec4_f32(v[0]);
        self.append_vec4_f32(v[1]);
        self.append_vec4_f32(v[2]);
        self.append_vec4_f32(v[3]);
    }
}
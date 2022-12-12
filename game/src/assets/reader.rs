use std::path::Path;

pub struct Reader(Vec<u8>, usize);
impl Reader {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(
            match std::fs::read(path.as_ref()) {
                Ok(v) => v,
                Err(e) => panic!("Can not read file: {}, error: {}", path.as_ref().display(), e)
            },
            0
        )
    }
    #[inline]
    pub fn finished(&mut self) -> bool {
        self.1 >= self.0.len()
    }
    #[inline]
    pub fn read_string(&mut self) -> String {
        let mut res = Vec::new();
        loop {
            match self.read_u8() {
                b'#' => break,
                byte => {
                    res.push(byte);
                }
            }
        }
        String::from_utf8_lossy(&res).to_string()
    }
    #[inline]
    pub fn read_end(&mut self, name: &String) {
        self.1 += 3;
        if self.0[self.1 - 3] != b'E' || self.0[self.1 - 2] != b'N' || self.0[self.1 - 1] != b'D' {
            panic!("Assest: '{}' corrupted, END not reached", name)
        }
    }

    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        self.1 += 1;
        self.0[self.1 - 1]
    }
    #[inline]
    pub fn read_u32(&mut self) -> u32 {
        self.1 += 4;
        u32::from_be_bytes([ self.0[self.1 - 4], self.0[self.1 - 3], self.0[self.1 - 2], self.0[self.1 - 1] ])
    }

    #[inline]
    pub fn read_vec3(&mut self) -> [f32;3] {
        self.1 += 12;
        [
            f32::from_be_bytes([ self.0[self.1 - 12], self.0[self.1 - 11], self.0[self.1 - 10], self.0[self.1 - 9] ]),
            f32::from_be_bytes([ self.0[self.1 - 8], self.0[self.1 - 7], self.0[self.1 - 6], self.0[self.1 - 5] ]),
            f32::from_be_bytes([ self.0[self.1 - 4], self.0[self.1 - 3], self.0[self.1 - 2], self.0[self.1 - 1] ])
        ]
    }
    #[inline]
    pub fn read_vec2(&mut self) -> [f32;2] {
        self.1 += 8;
        [
            f32::from_be_bytes([ self.0[self.1 - 8], self.0[self.1 - 7], self.0[self.1 - 6], self.0[self.1 - 5] ]),
            f32::from_be_bytes([ self.0[self.1 - 4], self.0[self.1 - 3], self.0[self.1 - 2], self.0[self.1 - 1] ])
        ]
    }
    #[inline]
    pub fn read_joints(&mut self) -> [u32;4] {
        self.1 += 4;
        [ self.0[self.1 - 4] as u32, self.0[self.1 - 3] as u32, self.0[self.1 - 2] as u32, self.0[self.1 - 1] as u32 ]
    }
    #[inline]
    pub fn read_vec4(&mut self) -> [f32;4] {
        self.1 += 16;
        [
            f32::from_be_bytes([ self.0[self.1 - 16], self.0[self.1 - 15], self.0[self.1 - 14], self.0[self.1 - 13] ]),
            f32::from_be_bytes([ self.0[self.1 - 12], self.0[self.1 - 11], self.0[self.1 - 10], self.0[self.1 - 9] ]),
            f32::from_be_bytes([ self.0[self.1 - 8], self.0[self.1 - 7], self.0[self.1 - 6], self.0[self.1 - 5] ]),
            f32::from_be_bytes([ self.0[self.1 - 4], self.0[self.1 - 3], self.0[self.1 - 2], self.0[self.1 - 1] ])
        ]
    }
    #[inline]
    pub fn read_mat4x4(&mut self) -> [[f32;4];4] {
        self.1 += 64;
        [
            [
                f32::from_be_bytes([ self.0[self.1 - 64], self.0[self.1 - 63], self.0[self.1 - 62], self.0[self.1 - 61] ]),
                f32::from_be_bytes([ self.0[self.1 - 60], self.0[self.1 - 59], self.0[self.1 - 58], self.0[self.1 - 57] ]),
                f32::from_be_bytes([ self.0[self.1 - 56], self.0[self.1 - 55], self.0[self.1 - 54], self.0[self.1 - 53] ]),
                f32::from_be_bytes([ self.0[self.1 - 52], self.0[self.1 - 51], self.0[self.1 - 50], self.0[self.1 - 49] ])
            ],
            [
                f32::from_be_bytes([ self.0[self.1 - 48], self.0[self.1 - 47], self.0[self.1 - 46], self.0[self.1 - 45] ]),
                f32::from_be_bytes([ self.0[self.1 - 44], self.0[self.1 - 43], self.0[self.1 - 42], self.0[self.1 - 41] ]),
                f32::from_be_bytes([ self.0[self.1 - 40], self.0[self.1 - 39], self.0[self.1 - 38], self.0[self.1 - 37] ]),
                f32::from_be_bytes([ self.0[self.1 - 36], self.0[self.1 - 35], self.0[self.1 - 34], self.0[self.1 - 33] ])
            ],
            [
                f32::from_be_bytes([ self.0[self.1 - 32], self.0[self.1 - 31], self.0[self.1 - 30], self.0[self.1 - 29] ]),
                f32::from_be_bytes([ self.0[self.1 - 28], self.0[self.1 - 27], self.0[self.1 - 26], self.0[self.1 - 25] ]),
                f32::from_be_bytes([ self.0[self.1 - 24], self.0[self.1 - 23], self.0[self.1 - 22], self.0[self.1 - 21] ]),
                f32::from_be_bytes([ self.0[self.1 - 20], self.0[self.1 - 19], self.0[self.1 - 18], self.0[self.1 - 17] ])
            ],
            [
                f32::from_be_bytes([ self.0[self.1 - 16], self.0[self.1 - 15], self.0[self.1 - 14], self.0[self.1 - 13] ]),
                f32::from_be_bytes([ self.0[self.1 - 12], self.0[self.1 - 11], self.0[self.1 - 10], self.0[self.1 - 9] ]),
                f32::from_be_bytes([ self.0[self.1 - 8], self.0[self.1 - 7], self.0[self.1 - 6], self.0[self.1 - 5] ]),
                f32::from_be_bytes([ self.0[self.1 - 4], self.0[self.1 - 3], self.0[self.1 - 2], self.0[self.1 - 1] ])
            ]
        ]
    }
}
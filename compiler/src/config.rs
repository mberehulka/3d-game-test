use std::path::Path;

pub enum VertexType {
    Basic, NUS, U, NU
}

pub struct Config {
    pub vertex_type: VertexType
}
impl Config {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().join("compile.conf");
        let mut res = Self {
            vertex_type: VertexType::Basic
        };
        let data = match std::fs::read_to_string(&path) {
            Ok(v) => v,
            Err(_) => return res
        };
        for line in data.lines() {
            let mut spl = line.split('='); 
            match spl.next().unwrap().trim() {
                "VertexType" => match spl.next().unwrap().trim() {
                    "NUS" => res.vertex_type = VertexType::NUS,
                    "NU" => res.vertex_type = VertexType::NU,
                    "U" => res.vertex_type = VertexType::U,
                    _ => {}
                },
                _ => {}
            }
        }
        res
    }
}
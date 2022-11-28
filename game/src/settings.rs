use std::io::Read;

use directories::UserDirs;
use json::{object, JsonValue};
use winit::dpi::{PhysicalSize, PhysicalPosition};

pub struct Settings {
    pub window_size: Option<PhysicalSize<u32>>,
    pub window_fullscreen: bool,
    pub window_decorations: bool,
    pub window_maximized: bool,
    pub window_position: Option<PhysicalPosition<u32>>,
    pub vsync: bool
}
impl Settings {
    pub fn read() -> Self {
        let user_dir = UserDirs::new().expect("Failed to get user directory");
        let doc_dir = user_dir.document_dir().expect("Failed to get user document directory");
        let set_dir = doc_dir.join("My Games/Nexodia/");
        let set_path = set_dir.join("settings.json");
        info!("Settings path: {set_path:?}");

        std::fs::create_dir_all(&set_dir).expect(&format!("Error creating directory: {set_dir:?}", ));
        
        let mut file = std::fs::OpenOptions::new().create(true).read(true).write(true).truncate(false).open(&set_path)
            .expect(&format!("Failed to open path: {set_path:?}"));
        
        let mut content = String::new();
        file.read_to_string(&mut content).expect(&format!("Error reading file: {set_path:?}"));

        if content.len() == 0 {
            let default = Self::get_default_json();
            default.write_pretty(&mut file, 4).expect(&format!("Error writing to: {set_path:?}"));
            return Self::from_json(default)
        }else {
            Self::from_json(json::parse(&content).unwrap())
        }
    }
    pub fn from_json(json: JsonValue) -> Self {
        const ERR: &'static str = "Invalid settings";
        info!("Settings loaded: {json}");
        let window = &json["window"];
        let size = &window["size"];
        let position = &window["position"];
        Self {
            window_size: if size.is_array() {
                Some(PhysicalSize {
                    width: size[0].as_u32().expect(ERR),
                    height: size[1].as_u32().expect(ERR)
                })
            } else { None },
            window_decorations: window["decorations"].as_bool().expect(ERR),
            window_fullscreen: window["fullscreen"].as_bool().expect(ERR),
            window_maximized: window["maximized"].as_bool().expect(ERR),
            window_position: if position.is_array() {
                Some(PhysicalPosition {
                    x: size[0].as_u32().expect(ERR),
                    y: size[1].as_u32().expect(ERR)
                })
            } else { None },
            vsync: window["vsync"].as_bool().expect(ERR)
        }
    }
    pub fn get_default_json() -> JsonValue {
        object! {
            window: {
                size: None::<bool>,
                fullscreen: false,
                decorations: true,
                maximized: true,
                position: None::<bool>,
                vsync: false
            }
        }
    }
}
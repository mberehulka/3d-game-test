use std::{path::Path, time::Instant};

use image::GenericImageView;

pub fn file(path: impl AsRef<Path>) -> Vec<u8> {
    let start = Instant::now();
    let mut res = crate::writer::Writer(Vec::new());
    res.0.push(b'I');

    let path = path.as_ref();
    let image = image::load_from_memory(&std::fs::read(path).unwrap()).unwrap();
    let rgb = image.to_rgb8();
    let dimensions = image.dimensions();
    res.append_string(path.with_extension("").file_name().unwrap().to_string_lossy().to_string());
    res.append_u32(dimensions.0);
    res.append_u32(dimensions.1);
    for rgb in rgb.pixels() {
        res.0.push(rgb[0]); res.0.push(rgb[1]); res.0.push(rgb[2]);
    }
    res.append_bytes(b"END");

    println!("texture:   {}, compiled in: {:.2} sec", path.clone().display(), (Instant::now() - start).as_secs_f64());
    res.0
}
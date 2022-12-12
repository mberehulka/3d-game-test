use std::{path::{Path, PathBuf}, sync::{Mutex, Arc, atomic::AtomicU16}, time::{Instant, Duration}, io::Write};

mod writer;
mod config;
mod gltf;
mod texture;
mod blender;

fn main() {
    println!("Compiling ...");
    let start = Instant::now();
    std::fs::create_dir("./assets/").err();

    let files = Arc::new(Mutex::new(Vec::new()));
    read_dir("./assets/", files.clone());

    let result = Arc::new(Mutex::new(Vec::new()));
    let threads = Arc::new(AtomicU16::new(0));
    for _ in 0..num_cpus::get() {
        let files = files.clone();
        let result = result.clone();
        let threads = threads.clone();
        threads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        std::thread::spawn(move || {
            loop {
                if files.lock().unwrap().len() == 0 { break }
                let file = files.lock().unwrap().remove(0);
                let ext = file.extension().unwrap().clone();
                match ext.to_str().unwrap() {
                    "gltf" | "glb" => {
                        let v = gltf::file(file);
                        result.lock().unwrap().push(v);
                    },
                    "png" | "jpeg" | "jpg" => {
                        let v = texture::file(file);
                        result.lock().unwrap().push(v);
                    },
                    _ => {}
                }
            }
            threads.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        });
    }
    while threads.load(std::sync::atomic::Ordering::SeqCst) > 0 {
        std::thread::sleep(Duration::from_millis(1));
    }

    let mut file = std::fs::OpenOptions::new().create(true).truncate(true).write(true).open("./assets/compiled.bin").unwrap();
    for bytes in result.lock().unwrap().iter() {
        file.write_all(bytes).unwrap();
    }
    
    blender::run_blender_compiler();

    println!("All assets compiled in: {:.2} sec", (Instant::now() - start).as_secs_f64());
}
fn read_dir(
    dir: impl AsRef<Path>,
    files: Arc<Mutex<Vec<PathBuf>>>
) {
    let dir = dir.as_ref();
    let dirs = match std::fs::read_dir(dir) {
        Ok(v) => v,
        Err(e) => panic!("can not access {} dir, error: {}", dir.display(), e)
    };
    for path in dirs {
        let path = path.unwrap().path();
        if path.is_dir() && path.file_name().unwrap() != "animations" {
            read_dir(path, files.clone())
        } else if path.is_file() {
            files.lock().unwrap().push(path.to_path_buf());
        }
    }
}
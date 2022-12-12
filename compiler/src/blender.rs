use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn run_blender_compiler() {
    std::fs::File::open("./compiler/compile.py").expect("./compiler/compile.py not found.");
    let comm = Command::new("blender")
        .args(["--background","--python","./compiler/compile.py"])
        .current_dir("./")
        .stdout(Stdio::piped())
        .spawn()
        .expect("blender command failed, try to download blender, add the executable path to the system environment variables, \
restart the computer and try again.");
    let mut f = BufReader::new(comm.stdout.unwrap());
    let mut found_error = false;
    loop {
        let mut buf = String::new();
        match f.read_line(&mut buf) {
            Ok(len) => {
                if len == 0 { break }
                let res = buf.as_str();
                if res.starts_with("Error") {
                    print!("{}", res);
                    found_error = true;
                }else if res.starts_with("animation") || found_error {
                    print!("{}", res);
                }
            }
            Err(e) => break println!("Error: {}", e)
        }
    }
}
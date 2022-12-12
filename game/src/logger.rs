use std::{panic, io::Write, fs::File, sync::Mutex};
use chrono::{Local, Timelike};
use env_logger::{Builder, WriteStyle};
use log::{LevelFilter, Level};

lazy_static! {
    static ref FILE: Mutex<File> = {
        let path = directories::UserDirs::new().unwrap().document_dir().unwrap().join("My Games/Nexodia/trace.log");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        Mutex::new(std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(path).unwrap())
    };
}

pub fn start() {
    let mut builder = Builder::new();
    builder
        .filter(None, LevelFilter::Trace)
        .filter(Some("wgpu_core"), LevelFilter::Info)
        .filter(Some("wgpu_core::device"), LevelFilter::Warn)
        .filter(Some("wgpu_hal"), LevelFilter::Info)
        .filter(Some("naga"), LevelFilter::Info)
        .format(|buf, record| {
            let level = record.level();
            let args = record.args();
            if level == Level::Trace {
                append_log(format!("{args}\n"));
                writeln!(buf, "\n\x1b[35m{args}\x1b[0m")
            } else {
                let now = Local::now();
                let timestamp = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
                let module = match record.module_path() { Some(v)=>v, None=>"" };
                let line = match record.line() { Some(v)=>v, None=>0 };
                let styled_level = buf.default_styled_level(level);
                append_log(format!("{timestamp} {styled_level} {module}:{line} {args}\r\n"));
                writeln!(buf, "\x1b[90m{timestamp} {styled_level} \x1b[96m{module}:{line}\x1b[0m {args}")
            }
        })
        .write_style(WriteStyle::Always)
        .init();
    panic::set_hook(Box::new(|panic_info| error!("{panic_info}")));
}

#[inline]
pub fn append_log(v: String) {
    FILE.lock().unwrap().write(v.as_bytes()).unwrap();
}
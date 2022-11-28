use winit::{window::{WindowBuilder, Fullscreen, Window}, event_loop::EventLoop, dpi::PhysicalPosition};
use crate::settings::Settings;

pub fn new(settings: &Settings, event_loop: &EventLoop<()>) -> Window {
    trace!("Creating window");
    let w = WindowBuilder::new()
        .with_title("Nexodia")
        .with_resizable(true)
        .with_decorations(settings.window_decorations)
        .build(&event_loop).unwrap();
    if settings.window_fullscreen {
        let monitor = w.current_monitor().unwrap();
        w.set_fullscreen(Some(Fullscreen::Exclusive({
            let mut r = None;
            if let Some(size) = settings.window_size {
                for video_mode in monitor.video_modes() {
                    if video_mode.size() == size {
                        r = Some(video_mode)
                    }
                }
            }
            if let None = r {
                let monitor_size = monitor.size();
                warn!("Size \"{:?}\" not supported, switching to monitor size: {monitor_size:?}", settings.window_size);
                for video_mode in monitor.video_modes() {
                    if video_mode.size() == monitor_size {
                        r = Some(video_mode)
                    }
                }
            }
            match r {
                Some(v) => v,
                None => {
                    let video_mode = monitor.video_modes().next().unwrap();
                    warn!("Current window size not supported, switching to: {video_mode:?}");
                    video_mode
                }
            }
        })))
    } else if settings.window_maximized {
        w.set_maximized(true)
    } else {
        let monitor = w.current_monitor().unwrap();
        let monitor_size = monitor.size();
        if let Some(size) = settings.window_size {
            w.set_inner_size(size)
        }
        if let Some(pos) = settings.window_position {
            w.set_outer_position(pos)
        } else {
            w.set_outer_position(PhysicalPosition {
                x: monitor_size.width as f32 / 2.,
                y: monitor_size.height as f32 / 2.
            })
        }
    }
    w.focus_window();
    w
}
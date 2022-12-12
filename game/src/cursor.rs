use std::sync::{atomic::AtomicBool, Mutex};

use winit::{window::Window, dpi::PhysicalPosition};

pub struct Cursor {
    pub active: AtomicBool,
    pub movement: Mutex<PhysicalPosition<f64>>,
    pub wheel_movement: Mutex<f32>
}
impl Cursor {
    pub fn new(window: &Window) -> Self {
        window.set_cursor_visible(false);
        let window_size = window.inner_size();
        window.set_cursor_position(PhysicalPosition {
            x: window_size.width as f64 / 2.,
            y: window_size.height as f64 / 2.
        }).unwrap();
        Self {
            active: AtomicBool::new(true),
            movement: Mutex::new(PhysicalPosition { x: 0., y: 0. }),
            wheel_movement: Mutex::new(0.)
        }
    }
    pub fn left(&self) {
        self.active.store(false, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn entered(&self) {
        self.active.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn moved(&self, window: &Window, position: PhysicalPosition<f64>) {
        if !self.active.load(std::sync::atomic::Ordering::Relaxed) { return }
        let window_size = window.inner_size();
        let w2 = window_size.width as f64 / 2.;
        let h2 = window_size.height as f64 / 2.;
        window.set_cursor_position(PhysicalPosition { x: w2, y: h2 }).unwrap();
        let mut movement = self.movement.lock().unwrap();
        movement.x += position.x - w2;
        movement.y += position.y - h2;
    }
    pub fn get_movement(&self) -> PhysicalPosition<f64> {
        let movement = *self.movement.lock().unwrap();
        drop(self.movement.lock().unwrap());
        *self.movement.lock().unwrap() = PhysicalPosition { x: 0., y: 0. };
        movement
    }
    pub fn wheel_moved(&self, v: f32) {
        *self.wheel_movement.lock().unwrap() += v
    }
    pub fn wheel_movement(&self) -> f32 {
        let wheel_movement = *self.wheel_movement.lock().unwrap();
        *self.wheel_movement.lock().unwrap() = 0.;
        wheel_movement
    }
}
use std::collections::HashSet;

use winit::{event_loop::{EventLoop, ControlFlow}, platform::run_return::EventLoopExtRunReturn,
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState}};

use crate::{context::Context, assets::Objects, ui::UI};

pub struct Game {
    pub event_loop: Option<EventLoop<()>>,
    pub context: Context,
    pub pressed_keys: HashSet<VirtualKeyCode>
}
impl Game {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let context = Context::new(&event_loop);
        Self {
            context,
            event_loop: Some(event_loop),
            pressed_keys: HashSet::new()
        }
    }
    pub fn start(mut self) {
        trace!("Start");
        let c = self.context;
        self.event_loop.take().unwrap().run_return(|event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state, .. }, .. } => {
                        match (key, state) {
                            (VirtualKeyCode::Escape, ElementState::Pressed) =>
                                *control_flow = ControlFlow::Exit,

                            (VirtualKeyCode::X, ElementState::Pressed) =>
                                c.lights.sun.rotate(0., 10., 0.),
                            (VirtualKeyCode::Z, ElementState::Pressed) =>
                                c.lights.sun.rotate(0., -10., 0.),
                            (VirtualKeyCode::V, ElementState::Pressed) =>
                                c.lights.sun.rotate(10., 0., 0.),
                            (VirtualKeyCode::C, ElementState::Pressed) =>
                                c.lights.sun.rotate(-10., 0., 0.),
                            
                            (key, ElementState::Pressed) => {self.pressed_keys.insert(key);},
                            (key, ElementState::Released) => {self.pressed_keys.remove(&key);}
                        }
                    },

                    WindowEvent::CursorLeft {..} => c.cursor.left(),
                    WindowEvent::CursorEntered {..} => c.cursor.entered(),
                    WindowEvent::Focused(focus) => if focus { c.cursor.entered() } else { c.cursor.left() },
                    WindowEvent::CursorMoved { position, .. } => c.cursor.moved(&c.window, position),
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => c.cursor.wheel_moved(x + y),
                        winit::event::MouseScrollDelta::PixelDelta(p) => c.cursor.wheel_moved((p.x + p.y) as f32)
                    },

                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_size) => c.resize(new_size),
                    _ => {}
                },
                Event::MainEventsCleared => c.window.request_redraw(),
                Event::RedrawRequested(_) => {
                    
                    c.lights.sun.update(&c.queue);
                    c.camera.lock().unwrap().update(&c.queue, &c.cursor);

                    Self::draw(&c);
                },
                _ => {}
            }
        });
        trace!("Dropping");
    }
    fn draw(c: &Context) {
        let mut encoder = c.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        
        c.lights.update(&c.queue);
        c.lights.draw(&c);

        let output_texture = match c.surface.get_current_texture() {
            Ok(v) => v,
            Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => return c.resize(c.window.inner_size()),
            Err(e) => panic!("Error getting current surface texture: {}", e)
        };
        let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let depth_texture = c.depth_texture.lock().unwrap();

        let objects = &c.objects.0.lock().unwrap();
        let camera = &c.camera.lock().unwrap();
        let squares = &c.ui.squares.lock().unwrap();
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true
                    }),
                    stencil_ops: None
                })
            });
            Objects::draw(&mut render_pass, c, objects, camera);
            UI::draw(&mut render_pass, c, squares);
        }

        c.queue.submit(std::iter::once(encoder.finish()));
        output_texture.present();
    }
}
use winit::{event_loop::{EventLoop, ControlFlow}, platform::run_return::EventLoopExtRunReturn,
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState}};

use crate::context::Context;

pub struct Game {
    pub event_loop: Option<EventLoop<()>>,
    pub context: Context
}
impl Game {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let context = Context::new(&event_loop);
        Self {
            context,
            event_loop: Some(event_loop)
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
                            _ => {}
                        }
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_size) => c.resize(new_size),
                    _ => {}
                },
                Event::MainEventsCleared => c.window.request_redraw(),
                Event::RedrawRequested(_) => {
                    let output_texture = match c.surface.get_current_texture() {
                        Ok(v) => v,
                        Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => return c.resize(c.window.inner_size()),
                        Err(e) => panic!("Error getting current surface texture: {}", e)
                    };
                    let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = c.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    
                    c.camera.lock().unwrap().update(&c.queue);

                    let depth_texture = c.depth_texture.lock().unwrap();
                    {
                        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                    }
                    c.queue.submit(std::iter::once(encoder.finish()));
                    output_texture.present();
                },
                _ => {}
            }
        });
        trace!("Droping");
    }
}
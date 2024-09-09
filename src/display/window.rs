use std::sync::Arc;
use std::time::Instant;

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use super::surface::SurfaceState;

pub trait Graphics {
    fn input(&mut self, active: bool, key: &str);
    fn update(&mut self, time: f32, delta: f32, fps: f32);
    fn render(&mut self, buffer: &mut [u8], width: u32, height: u32);
}

#[derive(Debug, Clone)]
pub struct WindowConfiguration {
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub resizable: bool,
    pub filter: bool,
    pub title: String,
}

impl WindowConfiguration {
    pub fn new(
        width: u32,
        height: u32,
        scale: f32,
        resizable: bool,
        filter: bool,
        title: String,
    ) -> Self {
        Self {
            width,
            height,
            scale,
            resizable,
            filter,
            title,
        }
    }
}

struct WindowTime {
    pub elapsed: f32,
    pub delta: f32,
    pub fps: f32,
    instant: Instant,
    previous: f32,
    total: f32,
    number: u32,
    samples: u32,
}

impl WindowTime {
    fn new() -> Self {
        Self {
            elapsed: 0.0,
            delta: 0.0,
            fps: 0.0,
            instant: Instant::now(),
            previous: 0.0,
            total: 0.0,
            number: 0,
            samples: 60,
        }
    }
    fn run(&mut self) {
        self.elapsed = self.instant.elapsed().as_secs_f32();
        self.delta = self.elapsed - self.previous;
        self.previous = self.elapsed;
        self.fps = 1.0 / self.delta;
        self.total += self.fps;
        self.number += 1;
    }
    fn print(&mut self) {
        if self.number == self.samples {
            println!("fps: {:.1}", self.total / self.number as f32);
            self.total = 0.0;
            self.number = 0;
        }
    }
}

struct WindowState<'a> {
    config: WindowConfiguration,
    window: Option<Arc<Window>>,
    surface: Option<SurfaceState<'a>>,
    graphics: Option<Box<dyn Graphics>>,
    time: WindowTime,
}

impl<'a> WindowState<'a> {
    fn new(config: WindowConfiguration) -> Self {
        Self {
            config,
            window: None,
            surface: None,
            graphics: None,
            time: WindowTime::new(),
        }
    }
}

pub fn run(
    width: u32,
    height: u32,
    scale: f32,
    resizable: bool,
    filter: bool,
    title: String,
    state: impl Graphics + 'static,
) {
    let event_loop = EventLoop::new().expect("Window event loop should instantiate");
    event_loop.set_control_flow(ControlFlow::Poll);
    // event_loop.set_control_flow(ControlFlow::Wait);
    let configuration = WindowConfiguration::new(width, height, scale, resizable, filter, title);
    let mut window_state = WindowState::new(configuration);
    window_state.graphics = Some(Box::new(state));
    event_loop
        .run_app(&mut window_state)
        .expect("Window event loop should run application with state");
}

impl<'a> ApplicationHandler for WindowState<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() || self.surface.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title(self.config.title.clone())
                            .with_resizable(self.config.resizable)
                            .with_inner_size(Size::Logical(LogicalSize::new(
                                self.config.scale as f64 * self.config.width as f64,
                                self.config.scale as f64 * self.config.height as f64,
                            ))),
                    )
                    .expect("Window should be created by event loop"),
            );

            self.window = Some(window.clone());

            let surface =
                pollster::block_on(SurfaceState::new(self.config.clone(), window.clone()));

            self.surface = Some(surface);

            window.request_redraw();
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.window.is_some() && self.surface.is_some() {
            if id != self.window.as_ref().expect("Window should exist").id() {
                return;
            }

            match event {
                WindowEvent::CloseRequested => {
                    let _ = self.window.take();
                    let _ = self.surface.take();
                    event_loop.exit();
                }
                WindowEvent::Resized(size) => {
                    let surface = self.surface.as_mut().expect("Window surface should exist");
                    surface.resize(size);
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            repeat: false,
                            ..
                        },
                    ..
                } => {
                    let _ = self.window.take();
                    let _ = self.surface.take();
                    event_loop.exit();
                }
                WindowEvent::KeyboardInput {
                    event,
                    is_synthetic: false,
                    ..
                } => {
                    if event.state.is_pressed() {
                        if let Key::Character(ch) = event.logical_key.as_ref() {
                            self.graphics
                                .as_mut()
                                .expect("Window graphics should exist")
                                .input(true, &ch);
                        }
                    } else {
                        if let Key::Character(ch) = event.logical_key.as_ref() {
                            self.graphics
                                .as_mut()
                                .expect("Window graphics should exist")
                                .input(false, &ch);
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    let surface = self.surface.as_mut().expect("Window surface should exist");

                    surface.window().request_redraw();

                    // let scale = surface.window().scale_factor();

                    // let (width, height) = {
                    //     let size = surface.window().inner_size();
                    //     (size.width, size.height)
                    // };

                    self.time.run();
                    self.time.print();

                    self.graphics
                        .as_mut()
                        .expect("Window graphics should exist")
                        .update(self.time.elapsed, self.time.delta, self.time.fps);

                    self.graphics
                        .as_mut()
                        .expect("Window graphics should exist")
                        .render(surface.buffer(), self.config.width, self.config.height);

                    match surface.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            surface.resize(surface.window().inner_size());
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            eprintln!("Out Of Memory");
                            let _ = self.window.take();
                            let _ = self.surface.take();
                            event_loop.exit();
                        }
                        Err(wgpu::SurfaceError::Timeout) => {
                            eprintln!("Surface Timeout");
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

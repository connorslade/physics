use std::num::NonZeroUsize;
use std::sync::Arc;

use nalgebra::Vector2;
use vello::kurbo::{stroke, Affine, Circle, PathEl, Point, Stroke, StrokeOpts};
use vello::peniko::{Color, Fill};
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use wgpu::{
    CompositeAlphaMode, Device, DeviceDescriptor, Instance, InstanceDescriptor, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::world::{FieldConfig, World};

const TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8Unorm;
const PARTICLE_RADIUS: f32 = 20.0;

pub struct Application<'a> {
    state: Option<State<'a>>,

    world: World,
    config: FieldConfig,
}

pub struct State<'a> {
    graphics: RenderContext<'a>,
    window_size: Vector2<f32>,

    mouse_down: bool,
    last_mouse: Vector2<f32>,
}

pub struct RenderContext<'a> {
    pub renderer: Renderer,

    pub window: Arc<Window>,
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("e_field | Connor Slade"))
                .unwrap(),
        );

        let instance = Instance::new(InstanceDescriptor::default());
        let adapter =
            pollster::block_on(instance.request_adapter(&RequestAdapterOptions::default()))
                .unwrap();

        let surface = instance.create_surface(window.clone()).unwrap();
        let (device, queue) =
            pollster::block_on(adapter.request_device(&DeviceDescriptor::default(), None)).unwrap();

        let renderer = Renderer::new(
            &device,
            RendererOptions {
                surface_format: Some(TEXTURE_FORMAT),
                use_cpu: false,
                antialiasing_support: AaSupport::all(),
                num_init_threads: NonZeroUsize::new(1),
            },
        )
        .unwrap();

        self.state = Some(State {
            window_size: self.world.size,
            mouse_down: false,
            last_mouse: Vector2::new(0.0, 0.0),
            graphics: RenderContext {
                renderer,

                window,
                surface,
                device,
                queue,
            },
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        if state.graphics.window.id() != id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                let size = Vector2::new(size.width as f32, size.height as f32);

                for (pos, _) in &mut self.world.particles {
                    *pos = pos.component_div(&state.window_size).component_mul(&size);
                }

                state.window_size = size;
                self.resize_surface();
            }
            WindowEvent::MouseInput {
                state: mouse,
                button: MouseButton::Left,
                ..
            } => {
                state.mouse_down = mouse == ElementState::Pressed;
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos = Vector2::new(position.x as f32, position.y as f32);
                let delta = pos - state.last_mouse;

                for (pos, _) in &mut self.world.particles {
                    let hovered = (*pos - state.last_mouse).magnitude() < PARTICLE_RADIUS;
                    if hovered && state.mouse_down {
                        *pos += delta;
                        break;
                    }
                }

                state.last_mouse = pos;
            }
            WindowEvent::RedrawRequested => {
                let physical_screen_size = state.graphics.window.inner_size();
                let screen_size = Vector2::new(
                    physical_screen_size.width as f32,
                    physical_screen_size.height as f32,
                );

                let mut scene = Scene::new();

                self.world.size = screen_size;
                for (pos, charge) in &self.world.particles {
                    let lines = self.world.generate_field_lines(&self.config, *pos, *charge);
                    let mut last = Vector2::new(0.0, 0.0);
                    let mut path = Vec::new();

                    for (start, end) in lines {
                        if last != start {
                            path.push(PathEl::MoveTo(Point::new(start.x as f64, start.y as f64)));
                        }
                        path.push(PathEl::LineTo(Point::new(end.x as f64, end.y as f64)));

                        last = end;
                    }

                    let path = stroke(
                        path,
                        &Stroke::new(self.config.line_width as f64),
                        &StrokeOpts::default(),
                        1.0,
                    );
                    scene.fill(Fill::NonZero, Affine::IDENTITY, Color::WHITE, None, &path);
                }

                for (pos, charge) in &self.world.particles {
                    let color = if *charge > 0 { Color::RED } else { Color::BLUE };
                    let shape = Circle::new((pos.x, pos.y), PARTICLE_RADIUS as f64);
                    scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &shape);
                }

                let surface_texture = state.graphics.surface.get_current_texture().unwrap();
                state
                    .graphics
                    .renderer
                    .render_to_surface(
                        &state.graphics.device,
                        &state.graphics.queue,
                        &scene,
                        &surface_texture,
                        &RenderParams {
                            base_color: Color::BLACK,
                            width: physical_screen_size.width,
                            height: physical_screen_size.height,
                            antialiasing_method: AaConfig::Msaa16,
                        },
                    )
                    .unwrap();
                surface_texture.present();

                state.graphics.window.request_redraw();
            }
            _ => (),
        }
    }
}

impl Application<'_> {
    pub fn new(world: World, config: FieldConfig) -> Self {
        Self {
            world,
            config,
            state: None,
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(self).unwrap();
    }

    fn resize_surface(&mut self) {
        let state = self.state.as_mut().unwrap();
        let size = state.graphics.window.inner_size();
        state.graphics.surface.configure(
            &state.graphics.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: TEXTURE_FORMAT,
                width: size.width,
                height: size.height,
                present_mode: PresentMode::AutoVsync,
                desired_maximum_frame_latency: 1,
                alpha_mode: CompositeAlphaMode::Opaque,
                view_formats: vec![],
            },
        );
    }
}

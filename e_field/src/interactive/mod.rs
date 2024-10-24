use std::num::NonZeroUsize;
use std::sync::Arc;

use nalgebra::Vector2;
use vello::peniko::Color;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use wgpu::{
    CompositeAlphaMode, Device, DeviceDescriptor, Instance, InstanceDescriptor, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::{Window, WindowId};

use crate::world::{FieldConfig, World};
use crate::Pos;

const TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8Unorm;
const PARTICLE_RADIUS: f32 = 27.0;

mod draw;
mod interface;

pub struct Application<'a> {
    state: Option<State<'a>>,

    world: World,
    config: FieldConfig,
}

pub struct State<'a> {
    graphics: RenderContext<'a>,
    window_size: Vector2<f32>,

    dragging: Option<usize>,
    mouse_down: [bool; 2],
    mouse: Vector2<f32>,
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
            dragging: None,
            mouse_down: [false; 2],
            mouse: Vector2::new(0.0, 0.0),
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

        let scale = state.graphics.window.scale_factor() as f32;
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
                button,
                ..
            } => {
                if button == MouseButton::Left {
                    state.mouse_down[0] = mouse == ElementState::Pressed;
                } else if button == MouseButton::Right {
                    state.mouse_down[1] = mouse == ElementState::Pressed;
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                state.mouse = Vector2::new(position.x as f32, position.y as f32)
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => interface::on_key(scale, state, &mut self.world, key),
            WindowEvent::RedrawRequested => {
                let physical_screen_size = state.graphics.window.inner_size();
                let screen_size = Vector2::new(
                    physical_screen_size.width as f32,
                    physical_screen_size.height as f32,
                );

                let mut scene = Scene::new();

                self.world.size = screen_size;

                interface::update(scale, state, &mut self.world);
                draw::draw(scale, &mut scene, &mut self.world, &self.config);

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

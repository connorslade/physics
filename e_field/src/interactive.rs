use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{self, Instant};

use vello::kurbo::{Affine, Circle};
use vello::low_level::Render;
use vello::peniko::{Color, Fill};
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use wgpu::{
    CompositeAlphaMode, Device, DeviceDescriptor, Instance, InstanceDescriptor, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::world::{FieldConfig, World};

const TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8Unorm;

pub struct Application<'a> {
    state: Option<State<'a>>,

    world: World,
    config: FieldConfig,
}

pub struct State<'a> {
    graphics: RenderContext<'a>,
    last_frame: Instant,
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
            graphics: RenderContext {
                renderer,

                window,
                surface,
                device,
                queue,
            },
            last_frame: Instant::now(),
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
            WindowEvent::Resized(_) => {
                self.resize_surface();
            }
            WindowEvent::RedrawRequested => {
                let now = time::Instant::now();
                let delta = now - state.last_frame;
                state.last_frame = now;
                println!("Frame time: {delta:?}");

                let mut scene = Scene::new();
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::rgb8(242, 140, 168),
                    None,
                    &Circle::new((420.0, 200.0), 120.0),
                );

                let surface_texture = state.graphics.surface.get_current_texture().unwrap();
                let screen_size = state.graphics.window.inner_size();
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
                            width: screen_size.width,
                            height: screen_size.height,
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

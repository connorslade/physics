use std::ops::RangeInclusive;

use anyhow::Result;
use compute::{
    buffer::UniformBuffer,
    export::{
        egui::{self, emath::Numeric, Context, Slider, Ui},
        nalgebra::Vector2,
        wgpu::{include_wgsl, RenderPass, ShaderStages},
        winit::window::WindowAttributes,
    },
    gpu::Gpu,
    interactive::{GraphicsCtx, Interactive},
    pipeline::render::RenderPipeline,
};
use encase::ShaderType;

struct App {
    uniform: UniformBuffer<Uniform>,
    render: RenderPipeline,

    ctx: Uniform,
}

#[derive(ShaderType, Default)]
struct Uniform {
    window: Vector2<u32>,
    scale: f32,
    position: Vector2<f32>,

    e_solutions: u32,
    v_solutions: u32,

    #[size(runtime)]
    particles: Vec<Particle>,
}

#[derive(ShaderType, Default)]
struct Particle {
    charge: i32,
    pos: Vector2<f32>,
}

impl Interactive for App {
    fn render(&mut self, gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        let screen = gcx.window.inner_size();
        self.ctx.window = Vector2::new(screen.width, screen.height);

        self.uniform.upload(&self.ctx).unwrap();
        self.render.draw_screen_quad(render_pass);
    }

    fn ui(&mut self, _gcx: GraphicsCtx, ctx: &Context) {
        let dragging_viewport = ctx.dragged_id().is_none();
        ctx.input(|input| {
            self.ctx.scale += input.smooth_scroll_delta.y / 200.0;
            if input.pointer.any_down() && dragging_viewport {
                let delta = input.pointer.delta();
                self.ctx.position += Vector2::new(-delta.x, delta.y);
            }
        });

        egui::Window::new("Electrostatics")
            .max_width(0.0)
            .resizable(false)
            .show(ctx, |ui| {
                dragger(ui, "Scale", &mut self.ctx.scale, 0.01..=1.0);

                ui.separator();

                dragger(ui, "E Solutions", &mut self.ctx.e_solutions, 0..=10);
                dragger(ui, "V Solutions", &mut self.ctx.v_solutions, 0..=10);
            });
    }
}

fn dragger<T: Numeric>(ui: &mut Ui, label: &str, value: &mut T, range: RangeInclusive<T>) {
    ui.horizontal(|ui| {
        ui.add(Slider::new(value, range));
        ui.label(label);
    });
}

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let uniform = gpu.create_uniform(Uniform::default()).unwrap();
    let render = gpu
        .render_pipeline(include_wgsl!("shader.wgsl"))
        .bind_buffer(&uniform, ShaderStages::FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Electrostatics"),
        App {
            uniform,
            render,

            ctx: Uniform {
                scale: 1.0,
                e_solutions: 5,
                v_solutions: 5,
                ..Default::default()
            },
        },
    )
    .run()?;

    Ok(())
}

use core::ops::RangeInclusive;
use std::time::Instant;

use compute::{
    buffer::{StorageBuffer, UniformBuffer},
    export::{
        egui::{emath::Numeric, Context, Slider, Ui, Window},
        nalgebra::{Vector2, Vector3},
        wgpu::RenderPass,
    },
    interactive::{GraphicsCtx, Interactive},
    misc::mutability::Mutable,
    pipeline::{compute::ComputePipeline, render::RenderPipeline},
};

use crate::{types::Particle, Uniform};

pub struct App {
    pub render: RenderPipeline,
    pub compute: ComputePipeline,

    pub ctx: Uniform,
    pub uniform: UniformBuffer<Uniform>,
    pub dots: StorageBuffer<Vec<Particle>, Mutable>,

    pub last_frame: Instant,
}

impl Interactive for App {
    fn ui(&mut self, _gcx: GraphicsCtx, ctx: &Context) {
        Window::new("Gravity")
            .default_width(0.0)
            .movable(false)
            .show(ctx, |ui| {
                ui.label(format!("Frame Time: {:.2?}", self.last_frame.elapsed()));
                self.last_frame = Instant::now();

                ui.separator();

                dragger(ui, "Radius", &mut self.ctx.radius, 0.0..=0.1);

                ui.separator();

                let mut dot_count = self.ctx.particles;
                dragger(ui, "Dots", &mut dot_count, 0..=65_535);

                if dot_count != self.ctx.particles {
                    self.ctx.particles = dot_count;
                    let dots = (0..dot_count)
                        .map(|_| Particle::random())
                        .collect::<Vec<_>>();
                    self.dots.upload(&dots).unwrap();
                }
            });
    }

    fn render(&mut self, gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        let screen = gcx.window.inner_size();
        self.ctx.window = Vector2::new(screen.width as f32, screen.height as f32);

        self.uniform.upload(&self.ctx).unwrap();
        self.compute
            .dispatch(Vector3::new(self.ctx.particles, 1, 1));
        self.render.draw_quad(render_pass, 0..self.ctx.particles);
    }
}

fn dragger<T: Numeric>(ui: &mut Ui, label: &str, value: &mut T, range: RangeInclusive<T>) {
    ui.horizontal(|ui| {
        ui.add(Slider::new(value, range));
        ui.label(label);
    });
}

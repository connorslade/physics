#![feature(get_many_mut)]

use consts::color;
use engine::{
    application::{Application, ApplicationArgs},
    drawable::shape::line::Line,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::WindowAttributes},
    },
    graphics_context::Drawable,
};
use itertools::Itertools;
use soft_body::SoftBody;

mod consts;
mod misc;
mod physics;
mod soft_body;

fn main() {
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default().with_title("Soft Body Physics"),
        asset_constructor: Box::new(|_constructor| {}),
        resumed: Box::new(|| {
            let mut soft_bodies = vec![SoftBody::circle()];

            Box::new(move |ctx| {
                let dt = ctx.delta_time * 5.0;
                ctx.background(color::BACKGROUND);

                let center = ctx.center();
                for body in soft_bodies.iter_mut() {
                    body.apply_force(dt, Vector2::y() * -200.0);
                    if ctx.input.mouse_down(MouseButton::Left) {
                        body.apply_force(dt, ctx.input.mouse - center);
                    }

                    body.tick(ctx, dt);
                    //     body.draw(ctx, center);

                    let mut control = body.points.iter().map(|x| x.position).collect::<Vec<_>>();

                    control.insert(0, control[control.len() - 1]);
                    control.push(control[1]);
                    control.push(control[0]);

                    for (a, b) in catmull_rom_strip(&control, 20, 0.5)
                        .into_iter()
                        .tuple_windows()
                    {
                        Line::new(a + center, b + center).draw(ctx);
                    }
                }
            })
        }),
        multisample: Some(4),
    })
    .run()
    .unwrap();
}

fn catmull_rom(
    out: &mut Vec<Vector2<f32>>,
    [p0, p1, p2, p3]: [Vector2<f32>; 4],
    samples: usize,
    alpha: f32,
) {
    let next_t = |t: f32, points: [Vector2<f32>; 2]| -> f32 {
        let dist = (points[0] - points[1]).magnitude();
        dist.powf(alpha) + t
    };

    let t0 = 0.0;
    let t1 = next_t(t0, [p0, p1]);
    let t2 = next_t(t1, [p1, p2]);
    let t3 = next_t(t2, [p2, p3]);

    for t in 0..=samples {
        let t = t1 + (t2 - t1) * (t as f32 / samples as f32);

        let a1 = (t1 - t) / (t1 - t0) * p0 + (t - t0) / (t1 - t0) * p1;
        let a2 = (t2 - t) / (t2 - t1) * p1 + (t - t1) / (t2 - t1) * p2;
        let a3 = (t3 - t) / (t3 - t2) * p2 + (t - t2) / (t3 - t2) * p3;
        let b1 = (t2 - t) / (t2 - t0) * a1 + (t - t0) / (t2 - t0) * a2;
        let b2 = (t3 - t) / (t3 - t1) * a2 + (t - t1) / (t3 - t1) * a3;
        let c = (t2 - t) / (t2 - t1) * b1 + (t - t1) / (t2 - t1) * b2;
        out.push(c);
    }
}

fn catmull_rom_strip(control: &[Vector2<f32>], samples: usize, alpha: f32) -> Vec<Vector2<f32>> {
    let segments = control.len() - 2;
    let mut out = Vec::with_capacity(samples * segments - 1);

    for (a, b, c, d) in control.iter().copied().tuple_windows() {
        catmull_rom(&mut out, [a, b, c, d], samples, alpha);
    }

    out
}

#![feature(get_many_mut)]

use catmull_rom::CatmullRom;
use consts::color;
use engine::{
    application::{Application, ApplicationArgs},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::WindowAttributes},
    },
    graphics_context::Drawable,
};
use soft_body::SoftBody;

mod catmull_rom;
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

                    let mut control = body
                        .border
                        .iter()
                        .map(|x| body.points[*x].position + center)
                        .collect::<Vec<_>>();

                    control.insert(0, control[control.len() - 1]);
                    control.push(control[1]);
                    control.push(control[2]);
                    control.push(control[3]);

                    CatmullRom::new(&control).thickness(16.0).draw(ctx);
                }
            })
        }),
        multisample: Some(4),
    })
    .run()
    .unwrap();
}

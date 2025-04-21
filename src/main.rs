#![feature(get_many_mut)]

use consts::color;
use engine::{
    application::{Application, ApplicationArgs},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::WindowAttributes},
    },
};
use physics::spring::Spring;
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
            let mut soft_bodies = [SoftBody::circle()];

            Box::new(move |ctx| {
                let dt = ctx.delta_time * 5.0;
                ctx.background(color::BACKGROUND);

                let center = ctx.center();
                for body in soft_bodies.iter_mut() {
                    body.apply_force(dt, Vector2::y() * -200.0);
                    if ctx.input.mouse_down(MouseButton::Left) {
                        for point in body.points.iter_mut() {
                            Spring::DEFAULT
                                .with_strength(4.0)
                                .with_damping(2.0)
                                .tick_one(point, ctx.input.mouse - center, dt);
                        }
                    }

                    body.tick(ctx, dt);
                    body.draw(ctx, center);
                    #[cfg(feature = "debug")]
                    body.debug(ctx);
                }
            })
        }),
        multisample: Some(4),
    })
    .run()
    .unwrap();
}

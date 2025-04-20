#![feature(get_many_mut)]

use consts::color;
use engine::{
    application::{Application, ApplicationArgs},
    exports::winit::{event::MouseButton, window::WindowAttributes},
};
use soft_body::SoftBody;

mod consts;
mod misc;
mod physics;
mod soft_body;

fn main() {
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default(),
        asset_constructor: Box::new(|_constructor| {}),
        resumed: Box::new(|| {
            let mut soft_bodies = vec![SoftBody::circle()];

            Box::new(move |ctx| {
                let dt = ctx.delta_time;
                ctx.background(color::BACKGROUND);

                let center = ctx.center();
                for body in soft_bodies.iter_mut() {
                    // body.apply_force(dt, Vector2::y() * -30.0);
                    if ctx.input.mouse_down(MouseButton::Left) {
                        body.apply_force(dt, ctx.input.mouse - center);
                    }

                    body.tick(ctx);
                    body.draw(ctx, center);
                }
            })
        }),
        multisample: Some(4),
    })
    .run()
    .unwrap();
}

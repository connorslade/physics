#![feature(get_many_mut)]

use engine::{
    application::{Application, ApplicationArgs},
    color::Rgb,
    exports::winit::window::WindowAttributes,
};
use soft_body::SoftBody;

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
                ctx.background(Rgb::repeat(0.235));

                let center = ctx.center();
                for body in soft_bodies.iter_mut() {
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

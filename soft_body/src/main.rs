#![feature(get_many_mut)]

use consts::color;
use engine::{
    application::{Application, ApplicationArgs},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode, window::WindowAttributes},
    },
};
use soft_body::SoftBody;
use spring::Spring;

mod catmull_rom;
mod consts;
mod repeat_first;
mod soft_body;
mod spring;

fn main() {
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default().with_title("Soft Body Physics"),
        asset_constructor: Box::new(|_constructor| {}),
        resumed: Box::new(|| {
            let mut soft_bodies = vec![SoftBody::circle()];
            let mut dragging = None;

            Box::new(move |ctx| {
                let dt = ctx.delta_time * 5.0;
                ctx.background(color::BACKGROUND);

                if ctx.input.key_pressed(KeyCode::KeyS) {
                    soft_bodies.push(SoftBody::circle());
                }

                let center = ctx.center();
                let mouse = ctx.input.mouse - center;
                let mouse_pressed = ctx.input.mouse_pressed(MouseButton::Left);
                if !ctx.input.mouse_down(MouseButton::Left) {
                    dragging = None;
                }

                for (idx, body) in soft_bodies.iter_mut().enumerate() {
                    body.apply_force(dt, Vector2::y() * -200.0);
                    if mouse_pressed && body.is_inside(mouse) {
                        dragging = Some(idx);
                    }

                    if dragging == Some(idx) {
                        for point in body.points.iter_mut() {
                            Spring::DEFAULT
                                .with_strength(4.0)
                                .with_damping(2.0)
                                .tick_one(point, mouse, dt);
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

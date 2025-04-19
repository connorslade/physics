#![feature(get_many_mut)]

use engine::{
    application::{Application, ApplicationArgs},
    color::Rgb,
    drawable::shape::{circle::Circle, line::Line},
    exports::{nalgebra::Vector2, winit::window::WindowAttributes},
    graphics_context::{Anchor, Drawable},
};
use itertools::Itertools;

struct Point {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
}

struct Constraint {
    points: [usize; 2],
    distance: f32,
}

struct SoftBody {
    points: Vec<Point>,
    constraints: Vec<Constraint>,
}

fn main() {
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default(),
        asset_constructor: Box::new(|_constructor| {}),
        resumed: Box::new(|| {
            let mut soft_bodies = vec![SoftBody {
                points: vec![
                    Point {
                        position: Vector2::new(0.0, 0.0),
                        velocity: Vector2::repeat(0.0),
                    },
                    Point {
                        position: Vector2::new(0.0, 100.0),
                        velocity: Vector2::repeat(0.0),
                    },
                    Point {
                        position: Vector2::new(100.0, 100.0),
                        velocity: Vector2::repeat(0.0),
                    },
                    Point {
                        position: Vector2::new(100.0, 0.0),
                        velocity: Vector2::repeat(0.0),
                    },
                ],
                constraints: vec![
                    Constraint {
                        points: [0, 2],
                        distance: 212.0,
                    },
                    Constraint {
                        points: [1, 3],
                        distance: 212.0,
                    },
                ],
            }];

            Box::new(move |ctx| {
                ctx.background(Rgb::repeat(0.235));

                let center = ctx.center();
                for body in soft_bodies.iter_mut() {
                    fn distance_constraint(
                        [a, b]: [&mut Point; 2],
                        distance: f32,
                        delta_time: f32,
                    ) {
                        let delta = b.position - a.position;
                        let (mag, dir) = (delta.magnitude(), delta.normalize());

                        let spring_force = (mag - distance) * dir;
                        let damping_force = (b.velocity - a.velocity).dot(&dir) * dir;

                        let force = spring_force + damping_force;
                        a.velocity += force * delta_time;
                        b.velocity -= force * delta_time;
                    }

                    for i in 0..body.points.len() {
                        let (a, b) = (i, (i + 1) % body.points.len());
                        let points = body.points.get_many_mut([a, b]).unwrap();
                        distance_constraint(points, 150.0, ctx.delta_time);
                    }

                    for constraint in body.constraints.iter() {
                        let points = body.points.get_many_mut(constraint.points).unwrap();
                        Line::new(points[0].position + center, points[1].position + center)
                            .thickness(4.0)
                            .color(Rgb::hex(0xFF0000))
                            .draw(ctx);

                        distance_constraint(points, constraint.distance, ctx.delta_time);
                    }

                    for point in body.points.iter_mut() {
                        point.velocity += Vector2::y() * -30.0 * ctx.delta_time;
                        point.position += point.velocity * ctx.delta_time;
                    }

                    for (a, b) in body
                        .points
                        .iter()
                        .cycle()
                        .take(body.points.len() + 1)
                        .tuple_windows()
                    {
                        Line::new(a.position + center, b.position + center)
                            .thickness(8.0)
                            .draw(ctx);
                    }

                    for point in body.points.iter() {
                        Circle::new(12.0)
                            .color(Rgb::repeat(0.0))
                            .position(point.position + center, Anchor::Center)
                            .draw(ctx);
                        Circle::new(8.0)
                            .position(point.position + center, Anchor::Center)
                            .draw(ctx);
                    }
                }
            })
        }),
        multisample: Some(4),
    })
    .run()
    .unwrap();
}

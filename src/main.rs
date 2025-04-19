#![feature(get_many_mut)]

use std::f32::consts::TAU;

use engine::{
    application::{Application, ApplicationArgs},
    color::Rgb,
    drawable::shape::{
        circle::Circle,
        line::{Line, LineCap},
    },
    exports::{nalgebra::Vector2, winit::window::WindowAttributes},
    graphics_context::{Anchor, Drawable},
};
use itertools::Itertools;

struct Point {
    initial: Vector2<f32>,
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
            let mut soft_bodies = vec![];

            soft_bodies.clear();

            let mut points = Vec::new();
            let mut constraints = Vec::new();

            let n = 8;
            let r = (150.0 * n as f32) / TAU;

            for i in 0..n {
                let t = i as f32 / n as f32 * TAU;
                let pos = Vector2::new(t.cos(), t.sin()) * r;
                points.push(Point {
                    initial: pos,
                    position: pos,
                    velocity: Vector2::zeros(),
                });

                let j = (i + n / 2) % n;
                constraints.push(Constraint {
                    points: [i, j],
                    distance: r * 2.0,
                });
            }

            soft_bodies.push(SoftBody {
                points,
                constraints,
            });

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

                        let floor = -center.y + 12.0;
                        if point.position.y < floor {
                            point.position.y = floor;
                            point.velocity.y = 0.0;
                        }
                    }

                    {
                        let mut com = Vector2::zeros();
                        let mut staring_com = Vector2::zeros();
                        for point in body.points.iter() {
                            com += point.position;
                            staring_com += point.initial;
                        }
                        com /= body.points.len() as f32;
                        staring_com /= body.points.len() as f32;

                        Circle::new(8.0)
                            .color(Rgb::hex(0x0000FF))
                            .position(com + center, Anchor::Center)
                            .z_index(1)
                            .draw(ctx);

                        for point in body.points.iter_mut() {
                            let pos = point.initial - staring_com + com;
                            distance_constraint(
                                [
                                    point,
                                    &mut Point {
                                        initial: Vector2::zeros(),
                                        position: pos,
                                        velocity: Vector2::zeros(),
                                    },
                                ],
                                0.0,
                                ctx.delta_time,
                            );
                            Circle::new(4.0)
                                .color(Rgb::hex(0x00FF00))
                                .position(pos + center, Anchor::Center)
                                .z_index(1)
                                .draw(ctx);
                            Line::new(pos + center, point.position + center)
                                .color(Rgb::hex(0xFF00))
                                .thickness(4.0)
                                .cap(LineCap::Round)
                                .z_index(1)
                                .draw(ctx);
                        }
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
                Line::new(Vector2::zeros(), Vector2::x() * ctx.size().x)
                    .thickness(8.0)
                    .draw(ctx);
            })
        }),
        multisample: Some(4),
    })
    .run()
    .unwrap();
}

use std::f32::consts::TAU;

use engine::{
    color::Rgb,
    drawable::shape::{
        circle::Circle,
        line::{Line, LineCap},
    },
    exports::nalgebra::{Rotation2, Vector2},
    graphics_context::{Anchor, Drawable, GraphicsContext},
};
use itertools::Itertools;

use crate::{
    misc::repeat_first::IteratorRepeatFirst,
    physics::{one_sided_spring, spring},
};

pub struct SoftBody {
    pub points: Vec<Point>,
    pub constraints: Vec<Constraint>,
}

pub struct Point {
    pub initial: Vector2<f32>,
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
}

pub struct Constraint {
    pub points: [usize; 2],
    pub distance: f32,
}

impl SoftBody {
    pub fn circle() -> Self {
        let mut points = Vec::new();
        let mut constraints = Vec::new();

        let n = 16;
        let r = 150.0;

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

        SoftBody {
            points,
            constraints,
        }
    }
}

impl SoftBody {
    pub fn draw(&self, ctx: &mut GraphicsContext, origin: Vector2<f32>) {
        for (a, b) in self.points.iter().repeat_first().tuple_windows() {
            Line::new(a.position + origin, b.position + origin)
                .thickness(8.0)
                .draw(ctx);
        }

        for point in self.points.iter() {
            Circle::new(12.0)
                .color(Rgb::repeat(0.0))
                .position(point.position + origin, Anchor::Center)
                .draw(ctx);
            Circle::new(8.0)
                .position(point.position + origin, Anchor::Center)
                .draw(ctx);
        }
    }
}

impl SoftBody {
    pub fn tick(&mut self, ctx: &mut GraphicsContext) {
        let dt = ctx.delta_time;

        for i in 0..self.points.len() {
            let (a, b) = (i, (i + 1) % self.points.len());
            let points = self.points.get_many_mut([a, b]).unwrap();
            let distance = (points[0].initial - points[1].initial).magnitude();
            spring(points, distance, dt);
        }

        for constraint in self.constraints.iter() {
            let points = self.points.get_many_mut(constraint.points).unwrap();
            spring(points, constraint.distance, dt);
        }

        self.shape_match(ctx);

        for point in self.points.iter_mut() {
            point.velocity -= Vector2::y() * 30.0 * dt;
            point.position += point.velocity * dt;

            let floor = -400.0;
            if point.position.y < floor {
                point.position.y = floor;
                point.velocity.y = 0.0;
            }
        }
    }

    fn shape_match(&mut self, ctx: &mut GraphicsContext) {
        let (dt, center) = (ctx.delta_time, ctx.center());

        let (mut com, mut staring_com) = (Vector2::zeros(), Vector2::zeros());
        for point in self.points.iter() {
            com += point.position;
            staring_com += point.initial;
        }
        com /= self.points.len() as f32;
        staring_com /= self.points.len() as f32;

        let mut angle = 0.0;
        for point in self.points.iter() {
            let (a, b) = (point.position - com, point.initial - staring_com);
            angle -= a.perp(&b).atan2(a.dot(&b));
        }
        angle /= self.points.len() as f32;

        for point in self.points.iter_mut() {
            let pos = (Rotation2::new(angle) * (point.initial - staring_com)) + com;

            if point.position != pos {
                one_sided_spring(point, pos, 0.0, dt);
            }

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
}

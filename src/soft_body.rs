use std::f32::consts::TAU;

use engine::{
    exports::nalgebra::{Rotation2, Vector2},
    graphics_context::{Drawable, GraphicsContext},
};
use itertools::Itertools;

use crate::{catmull_rom::CatmullRom, repeat_first::IteratorRepeatFirst, spring::Spring};

pub struct SoftBody {
    pub points: Vec<Point>,
    pub constraints: Vec<Constraint>,
    pub border: Vec<usize>,
}

pub struct Point {
    pub initial: Vector2<f32>,
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
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
                mass: 1.0,
            });

            let j = (i + n / 2) % n;
            constraints.push(Constraint {
                points: [i, j],
                distance: r * 2.0,
            });
        }

        SoftBody {
            border: (0..points.len()).collect(),
            points,
            constraints,
        }
    }

    pub fn apply_force(&mut self, dt: f32, force: Vector2<f32>) {
        for point in self.points.iter_mut() {
            point.velocity += force / point.mass * dt;
        }
    }
}

impl SoftBody {
    pub fn draw(&self, ctx: &mut GraphicsContext, origin: Vector2<f32>) {
        let mut control = self
            .border
            .iter()
            .map(|x| self.points[*x].position + origin)
            .collect::<Vec<_>>();

        // ↓ this ugly
        control.insert(0, control[control.len() - 1]);
        control.push(control[1]);
        control.push(control[2]);
        control.push(control[3]);

        CatmullRom::new(&control).thickness(16.0).draw(ctx);
    }

    pub fn is_inside(&self, point: Vector2<f32>) -> bool {
        let lines = self
            .border
            .iter()
            .map(|x| self.points[*x].position)
            .repeat_first()
            .tuple_windows()
            .collect::<Vec<_>>();
        intersect_lines(point, &lines) & 1 == 1
    }

    pub fn tick(&mut self, ctx: &mut GraphicsContext, dt: f32) {
        let center = ctx.center();

        for i in 0..self.points.len() {
            let (a, b) = (i, (i + 1) % self.points.len());
            let points = self.points.get_many_mut([a, b]).unwrap();
            let distance = (points[0].initial - points[1].initial).magnitude();
            Spring::DEFAULT.with_distance(distance).tick(points, dt);
        }

        for constraint in self.constraints.iter() {
            let points = self.points.get_many_mut(constraint.points).unwrap();
            Spring::DEFAULT
                .with_distance(constraint.distance)
                .tick(points, dt);
        }

        self.shape_match(dt);

        for point in self.points.iter_mut() {
            point.position += point.velocity * dt;

            for (axis, sign) in (0..=1).cartesian_product((0..=1).map(|x| (x * 2 - 1) as f32)) {
                let limit = center[axis];
                if point.position[axis] * sign > limit {
                    point.position[axis] = limit * sign;
                    point.velocity[axis] = 0.0;
                }
            }
        }
    }

    fn shape_match(&mut self, dt: f32) {
        let (com, staring_com, angle) = self.center_of_mass();
        for point in self.points.iter_mut() {
            let pos = (Rotation2::new(angle) * (point.initial - staring_com)) + com;
            Spring::DEFAULT.tick_one(point, pos, dt);
        }
    }

    fn center_of_mass(&self) -> (Vector2<f32>, Vector2<f32>, f32) {
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

        (com, staring_com, angle)
    }
    #[cfg(feature = "debug")]
    pub fn debug(&self, ctx: &mut GraphicsContext) {
        use crate::consts::color;
        use engine::{
            drawable::shape::{
                circle::Circle,
                line::{Line, LineCap},
            },
            graphics_context::Anchor,
        };

        let center = ctx.center();

        for constraint in self.constraints.iter() {
            Line::new(
                self.points[constraint.points[0]].position + center,
                self.points[constraint.points[1]].position + center,
            )
            .thickness(4.0)
            .color(color::RED)
            .draw(ctx);
        }

        let (com, staring_com, angle) = self.center_of_mass();

        Circle::new(8.0)
            .color(color::BLUE)
            .position(com + center, Anchor::Center)
            .z_index(1)
            .draw(ctx);

        for point in self.points.iter() {
            let pos = (Rotation2::new(angle) * (point.initial - staring_com)) + com;

            Circle::new(4.0)
                .color(color::GREEN)
                .position(pos + center, Anchor::Center)
                .z_index(1)
                .draw(ctx);
            Line::new(pos + center, point.position + center)
                .color(color::GREEN)
                .thickness(4.0)
                .cap(LineCap::Round)
                .z_index(1)
                .draw(ctx);
        }
    }
}

/// Checks how many lines are intersected by a horizontal line starting at
/// `start` and going right forever.
fn intersect_lines(start: Vector2<f32>, lines: &[(Vector2<f32>, Vector2<f32>)]) -> usize {
    let mut count = 0;

    // Find the intersection point between the lines y = start.y and y = mx + b
    // where the second line is defined by the two points in each element of
    // `lines`.
    //
    // m = (b.y - a.y) / (b.x - a.x)
    // b = a.y - m * a.x
    //
    // (start.y - b) / m = x
    for (a, b) in lines {
        let slope = (b.y - a.y) / (b.x - a.x);
        let offset = a.y - slope * a.x;
        let x = (start.y - offset) / slope;

        let is_inside = x >= start.x && ((x >= a.x && x <= b.x) || (x <= a.x && x >= b.x));
        count += is_inside as usize;
    }

    count
}

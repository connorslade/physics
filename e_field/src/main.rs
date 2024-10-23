use std::f32::consts::PI;

use nalgebra::Vector2;
use svg::{
    node::element::{path::Data, Circle, Path},
    Document,
};

type Pos = Vector2<f32>;

struct World {
    size: Pos,
    particles: Vec<(Pos, f32)>,
}

const LENGTH: f32 = 300.0;
const STEPS: usize = 10 * LENGTH as usize;
const OUT_LINES: usize = 6;
const LINE_WIDTH: f32 = 0.3;

const STEP: f32 = LENGTH / STEPS as f32;

fn main() {
    let mut world = World {
        size: Pos::new(100.0, 100.0),
        particles: vec![
            (Pos::new(40.0, 50.0), -3.0),
            (Pos::new(50.0, 50.0), 4.0),
            (Pos::new(60.0, 50.0), -3.0),
        ],
    };

    let mut document = Document::new().set("viewBox", (0, 0, world.size.x, world.size.y));

    for (pos, c) in world.particles.iter() {
        document = generate_field_lines(document, &world, *pos, *c, *c >= 1.0);
    }

    for (p, c) in &world.particles {
        let color = if *c > 0.0 { "red" } else { "blue" };
        let circle = Circle::new()
            .set("cx", p.x)
            .set("cy", p.y)
            .set("r", 1.0)
            .set("fill", color);
        document = document.add(circle);
    }

    svg::save("image.svg", &document).unwrap();
}

fn generate_field_lines(
    mut document: Document,
    world: &World,
    pos: Pos,
    charge: f32,
    is_positive: bool,
) -> Document {
    let out_lines = (OUT_LINES as f32 * charge.abs()).round();
    'line: for i in 0..(out_lines as usize) {
        let angle = 2.0 * PI * i as f32 / out_lines as f32 + PI / out_lines as f32;
        let angle_offset = Pos::new(angle.cos(), angle.sin()) * 0.9;
        let start = pos + angle_offset;

        let mut pos = Pos::new(start.x, start.y);
        let mut data = Data::new().move_to((pos.x, pos.y));
        for _ in 0..STEPS {
            let force = world.force_at(pos) * if is_positive { 1.0 } else { -1.0 };
            let new_pos = pos + force.normalize() * STEP;

            if !world.out_of_bounds(new_pos) {
                if world.out_of_bounds(pos) {
                    data = data.move_to((new_pos.x, new_pos.y));
                }

                data = data.line_to((new_pos.x, new_pos.y));
            }

            pos = new_pos;
            if world.at_particle(pos, 0.9) {
                if is_positive {
                    break;
                } else {
                    continue 'line;
                }
            };
        }

        document = document.add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", LINE_WIDTH)
                .set("d", data),
        );
    }

    document
}

impl World {
    pub fn force_at(&self, pos: Pos) -> Pos {
        let mut force = Pos::zeros();
        for (p, c) in &self.particles {
            let between = p - pos;
            let r = between.magnitude();
            let direction = between.normalize();

            force += direction * -(*c) / r.powi(2);
        }
        force
    }

    pub fn at_particle(&self, pos: Pos, cutoff: f32) -> bool {
        for (p, _) in &self.particles {
            if (p - pos).magnitude() < cutoff {
                return true;
            }
        }

        false
    }

    pub fn out_of_bounds(&self, pos: Pos) -> bool {
        pos.x < 0.0 || pos.x > self.size.x || pos.y < 0.0 || pos.y > self.size.y
    }
}

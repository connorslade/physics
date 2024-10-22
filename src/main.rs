use std::f32::consts::PI;

use nalgebra::Vector2;
use svg::{
    node::element::{path::Data, Circle, Path},
    Document,
};

type Pos = Vector2<f32>;

struct World {
    particles: Vec<(Pos, f32)>,
}

const LENGTH: f32 = 200.0;
const STEPS: usize = 20 * LENGTH as usize;
const OUT_LINES: usize = 12;

const STEP: f32 = LENGTH / STEPS as f32;

fn main() {
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.5);

    let world = World {
        particles: vec![
            (Pos::new(-10.0, 0.0), 1.0),
            (Pos::new(12.0, -23.0), -1.0),
            (Pos::new(0.0, 9.0), -1.0),
            (Pos::new(30.0, -14.0), 1.0),
        ],
    };

    let mut document = Document::new().set("viewBox", (-50, -50, 100, 100));

    for (pos, _) in world.particles.iter().filter(|x| x.1 > 0.0) {
        for i in 0..OUT_LINES {
            let angle = 2.0 * PI * i as f32 / OUT_LINES as f32 + PI / OUT_LINES as f32;
            let angle_offset = Pos::new(angle.cos(), angle.sin()) * 0.9;
            let start = pos + angle_offset;

            let mut pos = Pos::new(start.x, start.y);
            let mut data = Data::new().move_to((pos.x, pos.y));
            for _ in 0..STEPS {
                let force = world.force_at(pos);
                pos += force.normalize() * STEP;
                data = data.line_to((pos.x, pos.y));
            }

            document = document.add(path.clone().set("d", data));
        }
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
}

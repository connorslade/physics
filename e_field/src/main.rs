use interactive::Application;
use nalgebra::Vector2;
use world::{FieldConfig, World};

mod interactive;
mod svg;
mod world;

type Pos = Vector2<f32>;

fn main() {
    let world = World {
        size: Pos::new(100.0, 100.0),
        particles: vec![
            (Pos::new(40.0, 50.0), -3),
            (Pos::new(50.0, 50.0), 4),
            (Pos::new(60.0, 50.0), -3),
        ],
    };

    let config = FieldConfig {
        lines_per_charge: 6,
        line_width: 1.0,
        steps: 10 * 300,
        step: 1.0,
    };

    Application::new(world, config).run();

    // let document = svg::render(&world, &config);
    // ::svg::save("image.svg", &document).unwrap();
}

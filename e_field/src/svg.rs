use svg::{
    node::element::{path::Data, Circle, Path},
    Document,
};

use crate::{world::{FieldConfig, World}, Pos};

pub fn render(world: &World, config: &FieldConfig) -> Document {
    let mut document = Document::new().set("viewBox", (0, 0, world.size.x, world.size.y));

    let mut data = Data::new();
    let mut last = Pos::new(0.0, 0.0);

    for (pos, c) in world.particles.iter() {
        let lines = world.generate_field_lines(&config, *pos, *c);

        for (start, end) in lines {
            if last != start {
                data = data.move_to((start.x, start.y));
            }
            data = data.line_to((end.x, end.y));

            last = end;
        }
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", config.line_width)
        .set("d", data);
    document = document.add(path);

    for (p, c) in &world.particles {
        let color = if *c > 0.0 { "red" } else { "blue" };
        let circle = Circle::new()
            .set("cx", p.x)
            .set("cy", p.y)
            .set("r", 1.0)
            .set("fill", color);
        document = document.add(circle);
    }

    document
}
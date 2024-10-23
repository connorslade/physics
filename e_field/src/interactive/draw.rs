use nalgebra::Vector2;
use vello::{
    kurbo::{stroke, Affine, Circle, PathEl, Point, Stroke, StrokeOpts},
    peniko::{Color, Fill},
    Scene,
};

use crate::world::{FieldConfig, World};

use super::PARTICLE_RADIUS;

pub fn draw(scene: &mut Scene, world: &mut World, config: &FieldConfig) {
    for (pos, charge) in &world.particles {
        let lines = world.generate_field_lines(config, *pos, *charge);
        let mut last = Vector2::new(0.0, 0.0);
        let mut path = Vec::new();

        for (start, end) in lines {
            if last != start {
                path.push(PathEl::MoveTo(Point::new(start.x as f64, start.y as f64)));
            }
            path.push(PathEl::LineTo(Point::new(end.x as f64, end.y as f64)));

            last = end;
        }

        let path = stroke(
            path,
            &Stroke::new(config.line_width as f64),
            &StrokeOpts::default(),
            1.0,
        );
        scene.fill(Fill::NonZero, Affine::IDENTITY, Color::WHITE, None, &path);
    }

    for (pos, charge) in &world.particles {
        let color = if *charge > 0 { Color::RED } else { Color::BLUE };
        let shape = Circle::new((pos.x, pos.y), PARTICLE_RADIUS as f64);
        scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &shape);
    }
}

use std::sync::Arc;

use nalgebra::Vector2;
use once_cell::sync::Lazy;
use vello::{
    kurbo::{stroke, Affine, Circle, PathEl, Point, Stroke, StrokeOpts},
    peniko::{Blob, Color, Fill, Font},
    skrifa::{prelude::Size, FontRef, MetadataProvider},
    Glyph, Scene,
};

use crate::world::{FieldConfig, World};

use super::PARTICLE_RADIUS;

static FONT: Lazy<Font> = Lazy::new(|| {
    let font_data = include_bytes!("../../assets/JetBrainsMono-Regular.ttf");
    Font::new(Blob::new(Arc::new(font_data)), 0)
});

static FONT_REF: Lazy<FontRef> = Lazy::new(|| FontRef::new(FONT.data.data()).unwrap());

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

        let text = format!(
            "{}{}",
            if *charge > 0 { "+" } else { "-" },
            charge.unsigned_abs()
        );

        let axes = FONT_REF.axes();
        let variations: &[(&str, f32)] = &[];
        let var_loc = axes.location(variations);
        let metrics = FONT_REF.metrics(Size::new(16.0), &var_loc);
        let glyph_metrics = FONT_REF.glyph_metrics(Size::new(16.0), &var_loc);

        let text_height = metrics.ascent + metrics.descent;
        let text_width = text.chars().fold(0.0, |acc, x| {
            acc + glyph_metrics
                .advance_width(FONT_REF.charmap().map(x).unwrap_or_default())
                .unwrap_or_default()
        });

        let mut x_offset = -text_width / 2.0;

        scene.draw_glyphs(&FONT).brush(Color::WHITE).draw(
            Fill::EvenOdd,
            text.chars().map(|x| {
                let gid = FONT_REF.charmap().map(x).unwrap_or_default();
                let (x, y) = (pos.x + x_offset, pos.y + text_height / 2.0);
                x_offset += glyph_metrics.advance_width(gid).unwrap_or_default();
                Glyph {
                    id: gid.to_u32(),
                    x,
                    y,
                }
            }),
        );
    }
}

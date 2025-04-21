use engine::{
    color::Rgb,
    exports::nalgebra::Vector2,
    graphics_context::{Drawable, GraphicsContext},
    render::shape::ShapeVertex,
};
use itertools::Itertools;

pub struct CatmullRom<'a> {
    points: &'a [Vector2<f32>],
    thickness: f32,
    precision: usize,
    alpha: f32,

    z_index: i16,
    color: Rgb<f32>,
}

impl<'a> CatmullRom<'a> {
    pub fn new(points: &'a [Vector2<f32>]) -> Self {
        Self {
            points,
            thickness: 4.0,
            precision: 16,
            alpha: 0.5,

            z_index: 0,
            color: Rgb::repeat(1.0),
        }
    }

    pub fn thickness(self, thickness: f32) -> Self {
        Self { thickness, ..self }
    }

    pub fn precision(self, precision: usize) -> Self {
        Self { precision, ..self }
    }

    pub fn alpha(self, alpha: f32) -> Self {
        Self { alpha, ..self }
    }

    pub fn z_index(self, z_index: i16) -> Self {
        Self { z_index, ..self }
    }

    pub fn color(self, color: Rgb<f32>) -> Self {
        Self { color, ..self }
    }
}

impl<'a> Drawable for CatmullRom<'a> {
    fn draw(self, ctx: &mut GraphicsContext) {
        let points = catmull_rom_strip(&self.points, self.precision, self.alpha);

        let mut double = Vec::new();
        for i in 0..points.len() - 1 {
            let delta = (points[i.saturating_sub(1)] - points[i + 1]).normalize();
            let perp = Vector2::new(delta.y, -delta.x) * self.thickness / 2.0;

            let mut push_vert = |pos| {
                ctx.shapes.push_vertex(ShapeVertex {
                    position: pos,
                    z_index: self.z_index,
                    color: self.color,
                })
            };

            let point = points[i];
            double.push((push_vert(point + perp), push_vert(point - perp)));
        }

        for (a, b) in double.into_iter().tuple_windows() {
            ctx.shapes.push_quad([a.0, a.1, b.1, b.0]);
        }
    }
}

fn catmull_rom(
    out: &mut Vec<Vector2<f32>>,
    [p0, p1, p2, p3]: [Vector2<f32>; 4],
    samples: usize,
    alpha: f32,
) {
    let next_t = |t: f32, points: [Vector2<f32>; 2]| -> f32 {
        let dist = (points[0] - points[1]).magnitude();
        dist.powf(alpha) + t
    };

    let t0 = 0.0;
    let t1 = next_t(t0, [p0, p1]);
    let t2 = next_t(t1, [p1, p2]);
    let t3 = next_t(t2, [p2, p3]);

    for t in 0..=samples {
        let t = t1 + (t2 - t1) * (t as f32 / samples as f32);

        let a1 = (t1 - t) / (t1 - t0) * p0 + (t - t0) / (t1 - t0) * p1;
        let a2 = (t2 - t) / (t2 - t1) * p1 + (t - t1) / (t2 - t1) * p2;
        let a3 = (t3 - t) / (t3 - t2) * p2 + (t - t2) / (t3 - t2) * p3;
        let b1 = (t2 - t) / (t2 - t0) * a1 + (t - t0) / (t2 - t0) * a2;
        let b2 = (t3 - t) / (t3 - t1) * a2 + (t - t1) / (t3 - t1) * a3;
        let c = (t2 - t) / (t2 - t1) * b1 + (t - t1) / (t2 - t1) * b2;
        out.push(c);
    }
}

fn catmull_rom_strip(control: &[Vector2<f32>], samples: usize, alpha: f32) -> Vec<Vector2<f32>> {
    let segments = control.len() - 2;
    let mut out = Vec::with_capacity(samples * segments - 1);

    for (a, b, c, d) in control.iter().copied().tuple_windows() {
        catmull_rom(&mut out, [a, b, c, d], samples, alpha);
    }

    out
}

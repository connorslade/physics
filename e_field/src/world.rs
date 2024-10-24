use std::f32::consts::PI;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::Pos;

pub struct World {
    pub size: Pos,
    pub particles: Vec<(Pos, i32)>,
}

pub struct FieldConfig {
    pub lines_per_charge: usize,
    pub line_width: f32,
    pub steps: usize,
    pub step: f32,
}

impl World {
    pub fn force_at(&self, pos: Pos) -> Pos {
        let mut force = Pos::zeros();
        for (p, c) in &self.particles {
            let between = p - pos;
            let r = between.magnitude();
            let direction = between.normalize();

            force += direction * -(*c as f32) / r.powi(2);
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

    pub fn generate_field_lines(
        &self,
        config: &FieldConfig,
        pos: Pos,
        charge: i32,
    ) -> Vec<(Pos, Pos)> {
        let out_lines = config.lines_per_charge * charge.unsigned_abs() as usize;
        let is_positive = charge > 0;

        (0..out_lines)
            .par_bridge()
            .filter_map(|i| {
                let angle = 2.0 * PI * i as f32 / out_lines as f32 + PI / out_lines as f32;
                let angle_offset = Pos::new(angle.cos(), angle.sin()) * 0.9;
                let start = pos + angle_offset;

                let mut pos = Pos::new(start.x, start.y);
                let mut line = Vec::new();

                for _ in 0..config.steps {
                    let was_oob = self.out_of_bounds(pos);
                    let force = self.force_at(pos) * if is_positive { 1.0 } else { -1.0 };
                    let new_pos =
                        pos + force.normalize() * config.step * if was_oob { 10.0 } else { 1.0 };

                    if !self.out_of_bounds(new_pos) && !was_oob {
                        line.push((pos, new_pos));
                    }

                    pos = new_pos;
                    if self.at_particle(pos, 0.9) {
                        if is_positive {
                            break;
                        } else {
                            // Returning None to indicate skipping to the next iteration
                            return None;
                        }
                    }
                }

                Some(line)
            })
            .flatten()
            .collect()
    }
}

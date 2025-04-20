use engine::exports::nalgebra::Vector2;

use crate::soft_body::Point;

#[derive(Clone, Copy)]
pub struct Spring {
    distance: f32,
    strength: f32,
    damping: f32,
}

impl Spring {
    pub const DEFAULT: Self = Self {
        distance: 0.0,
        strength: 3.0,
        damping: 1.0,
    };

    pub fn with_distance(self, distance: f32) -> Self {
        Self { distance, ..self }
    }

    pub fn with_strength(self, strength: f32) -> Self {
        Self { strength, ..self }
    }

    pub fn with_damping(self, damping: f32) -> Self {
        Self { damping, ..self }
    }

    pub fn tick(&self, [a, b]: [&mut Point; 2], dt: f32) {
        let delta = a.position - b.position;
        let delta_x = delta.magnitude() - self.distance;
        let basis = delta.try_normalize(0.0).unwrap_or_default();
        let spring = basis * self.strength * delta_x;

        let delta_v = a.velocity - b.velocity;
        let damping = self.damping * delta_v;

        let force = spring + damping;
        a.velocity -= force / a.mass * dt;
        b.velocity += force / b.mass * dt;
    }

    pub fn tick_one(&self, a: &mut Point, b: Vector2<f32>, dt: f32) {
        let delta = a.position - b;
        let delta_x = delta.magnitude() - self.distance;
        let basis = delta.try_normalize(0.0).unwrap_or_default();

        let spring = basis * self.strength * delta_x;
        let damping = self.damping * a.velocity;

        let force = spring + damping;
        a.velocity -= force / a.mass * dt;
    }
}

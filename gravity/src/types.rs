use std::f32::consts::PI;

use compute::export::nalgebra::Vector2;
use encase::ShaderType;
use rand::{thread_rng, Rng};

#[derive(ShaderType)]
pub struct Particle {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    mass: f32,
}

#[derive(ShaderType)]
pub struct Uniform {
    pub window: Vector2<f32>,
    pub dt: f32,

    pub particles: u32,
    pub radius: f32,
}

impl Particle {
    pub fn random() -> Self {
        let mut rand = thread_rng();

        let t = rand.gen::<f32>() * 2.0 * PI;

        let position = Vector2::new(t.sin(), t.cos()) / 4.0 + Vector2::repeat(0.5);
        let velocity = Vector2::new(t.cos(), -t.sin());

        Self {
            position,
            velocity,
            mass: 1.0,
        }
    }
}

impl Default for Uniform {
    fn default() -> Self {
        Self {
            window: Vector2::zeros(),
            dt: 0.0001,

            particles: 0,
            radius: 0.001,
        }
    }
}

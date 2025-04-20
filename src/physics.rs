use engine::exports::nalgebra::Vector2;

use crate::soft_body::Point;

pub fn spring([a, b]: [&mut Point; 2], distance: f32, dt: f32) {
    let delta = b.position - a.position;
    let (mag, dir) = (delta.magnitude(), delta.normalize());

    let spring_force = (mag - distance) * dir;
    let damping_force = (b.velocity - a.velocity).dot(&dir) * dir;

    let force = spring_force + damping_force;
    a.velocity += force * dt;
    b.velocity -= force * dt;
}

pub fn one_sided_spring(a: &mut Point, b: Vector2<f32>, distance: f32, dt: f32) {
    let delta = b - a.position;
    let (mag, dir) = (delta.magnitude(), delta.normalize());

    let spring_force = (mag - distance) * dir;
    let damping_force = a.velocity.dot(&dir) * dir;

    let force = spring_force - damping_force;
    a.velocity += force * dt;
}

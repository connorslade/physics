@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1) var<uniform> ctx: Uniform;

struct Uniform {
    window: vec2f,
    dt: f32,

    particles: u32,
    radius: f32,
}

struct Particle {
    position: vec2f,
    velocity: vec2f,
    mass: f32
}

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let a = particles[id.x];

    let object = vec2(0.4, 0.5);
    let object2 = vec2(0.6, 0.5);

    let diff = object - a.position;
    let diff2 = object2 - a.position;

    let force = normalize(diff) * ctx.dt * (a.mass * 1.0) / dot(diff, diff)
                + normalize(diff2) * ctx.dt * (a.mass * 1.0) / dot(diff2, diff2);

    let acceleration = force / a.mass;
    particles[id.x].velocity += acceleration;
    particles[id.x].position += particles[id.x].velocity * ctx.dt;
}

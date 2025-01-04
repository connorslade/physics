struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(1) uv: vec2f,
};

@vertex
fn vert(
    @location(0) pos: vec4f,
    @location(1) uv: vec2f,
) -> VertexOutput {
    return VertexOutput(pos, uv);
}

@group(0) @binding(0) var<storage, read> ctx: Uniform;

struct Uniform {
    window: vec2u,
    scale: f32,
    position: vec2f,

    e_solutions: u32,
    v_solutions: u32,

    particles: array<Particle>
}

struct Particle {
    charge: i32,
    position: vec2f
}

const particle_radius: f32 = 0.1;
const fade_radius: f32 = 0.05;
const line_thickness: f32 = 1.5;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4f {
    var window = vec2f(ctx.window);
    var scale = min(window.x, window.y);

    var offset = ctx.position / scale;
    var pos = ((in.uv - 0.5 + offset) / ctx.scale + 0.5) * window;

    var field = vec2(0.0);
    for (var i = u32(0); i < arrayLength(&ctx.particles); i++) {
        var particle = ctx.particles[i];
        var delta = pos - window * particle.position;
        field += f32(particle.charge) * cLog(delta);
    }

    var e = cos(field.y * f32(ctx.e_solutions));
    var e_value = line_thickness - abs(e / fwidth(e));

    var v = cos(field.x * f32(ctx.v_solutions));
    var v_value = line_thickness - abs(v / fwidth(v));

    var e_color = max(e_value, 0.0) * vec3(0.031, 0.482, 0.737);
    var v_color = max(v_value, 0.0) * vec3(0.031, 0.596, 0.490);
    var frag = vec4(max(e_color, v_color), 1.0);

    for (var i = u32(0); i < arrayLength(&ctx.particles); i++) {
        var particle = ctx.particles[i];

        var delta = pos - window * particle.position;
        var color = mix(vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), f32(particle.charge < 0));

        var t = -(length(delta) - particle_radius * scale) / (fade_radius * scale);
        frag = mix(frag, vec4(color, 1.0), smoothstep(0.0, 1.0, t));
    }

    return frag;
}

fn cLog(z: vec2f) -> vec2f {
    return vec2(log(length(z)), atan2(z.y, z.x));
}
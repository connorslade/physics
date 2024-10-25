// VERTEX SHADER //

struct VertexOutput {
    @builtin(position)
    position: vec4<f32>,

    @location(0)
    tex_coord: vec2<f32>,
};

@vertex
fn vert(
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = position;
    out.tex_coord = tex_coord;
    return out;
}

// FRAGMENT SHADER //

var<private> charge: array<f32, 3> = array<f32, 3>(-2.0, -2.0, 2.0);
const count: u32 = 3;
const solutions: u32 = 2;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let size: vec2<f32> = vec2<f32>(1.0);
    var pos: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
        vec2(size.x * 0.3, size.y / 2.0),
        vec2(size.x * 0.7, size.y / 2.0),
        vec2(size.x * 0.5, size.y / 2.0),
    );

    var net = 0.0;
    for (var i: u32 = 0; i < count; i++) {
        let deta = in.tex_coord - pos[i];
        net += charge[i] * atan2(deta.y, deta.x);
    }

    let a = sin(net * f32(solutions));
    let value = abs(a / fwidth(a));
    return vec4<f32>(2.0 -vec3(value), 1.0);
}

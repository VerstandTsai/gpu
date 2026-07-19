struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) pos2d: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(model.position, 1.0);
    out.pos2d = model.position.xy;
    return out;
}

fn get_color(pos2d: vec2<f32>) -> f32 {
    return sin(64 * pos2d.x) * cos(64 * pos2d.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(vec3<f32>(pow(get_color(in.pos2d) / 2 + 0.5, 2.2)), 1.0);
}


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords.xy;
    return out;
}

fn get_color(tex_coords: vec2<f32>) -> f32 {
    return sin(64 * tex_coords.x) * cos(64 * tex_coords.y) / 2 + 0.5;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(vec3<f32>(pow(get_color(in.tex_coords), 2.2)), 1.0);
}


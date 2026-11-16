const positions = array(
    vec4(0., 1., 0., 1.),
    vec4(-1., -1., 0., 1.),
    vec4(1., -1., 0., 1.)
);
const colors = array(
    vec4(0., 1., 0., 1.),
    vec4(0., 0., 1., 1.),
    vec4(1., 0., 0., 1.)
);

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    output.position = positions[vertex_index];
    output.color = colors[vertex_index] * vec4(1.0, 1.0, 0.0, 1.0);
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color * vec4(1.0, 0.0, 1.0, 1.0);
}

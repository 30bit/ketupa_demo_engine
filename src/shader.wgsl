struct ParamsUniform {
    screen_half_recip: vec2<f32>;
    screen_zoom: f32;
    dummy: u32;
};

[[group(0), binding(0)]]
var<uniform> params: ParamsUniform;

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] scale_and_rotation: vec4<f32>;
    [[location(2)]] translation_color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position.x = (dot(in.scale_and_rotation.xz, in.position)
        + in.translation_color.x) 
        * params.screen_half_recip.x;
    out.clip_position.y = (dot(in.scale_and_rotation.yw, in.position)
        + in.translation_color.y)
        * params.screen_half_recip.y;
    out.clip_position.w = params.screen_zoom;
    out.color = unpack4x8unorm(bitcast<u32>(in.translation_color.z));
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}
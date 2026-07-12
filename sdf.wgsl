struct ShaderSdfRect {
    width: f32,
    height: f32,
    scale: f32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    radius: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    border_width: f32,
    border_r: f32,
    border_g: f32,
    border_b: f32,
    border_a: f32,
    shadow_offset_x: f32,
    shadow_offset_y: f32,
    shadow_blur: f32,
    shadow_r: f32,
    shadow_g: f32,
    shadow_b: f32,
    shadow_a: f32,
    clip_start: u32,
    clip_count: u32,
};

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) inst: u32,
};

@group(0) @binding(0) var<storage> u: array<ShaderSdfRect>;

@vertex
fn vs_main(@builtin(vertex_index) i: u32, @builtin(instance_index) inst: u32) -> VsOut {
    var pos = array<vec2<f32>, 6>(
        vec2(-u[inst].width, u[inst].height), vec2(-u[inst].width, -u[inst].height), vec2(u[inst].width, -u[inst].height),
        vec2(-u[inst].width, u[inst].height), vec2(u[inst].width, -u[inst].height), vec2(u[inst].width, u[inst].height),
    );

    let cord = vec2<f32>(u[inst].x, u[inst].y);

    var out: VsOut;
    out.pos = vec4<f32>((pos[i] * u[inst].scale) + cord, u[inst].z, u[inst].w);
    out.inst = inst;

    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(u[in.inst].r, u[in.inst].g, u[in.inst].b, u[in.inst].a);
}

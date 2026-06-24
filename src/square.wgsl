struct ShaderSquare {
    width: f32,
    height: f32,
    scale: f32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,

    _pad:u32
};
struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) inst: u32,
};

@group(0) @binding(0) var<storage> u: array<ShaderSquare>;

@vertex
fn vs_main(@builtin(vertex_index) i: u32, @builtin(instance_index) inst: u32) -> VsOut {
    var pos = array<vec2<f32>, 6>(
        vec2(-u[inst].width,  u[inst].height), vec2(-u[inst].width, -u[inst].height), vec2(u[inst].width, -u[inst].height),
        vec2(-u[inst].width,  u[inst].height), vec2( u[inst].width, -u[inst].height), vec2(u[inst].width,  u[inst].height),
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

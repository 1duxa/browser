#[derive(encase::ShaderType, Default)]
pub struct ShaderSquare {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,

    pub _pad: u32,
}
impl ShaderSquare {
    pub const VERTECIES: u32 = 2;
}

#[derive(encase::ShaderType, Default)]
pub struct ShaderSdfRect {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,

    pub radius: f32,

    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,

    pub border_width: f32,
    pub border_r: f32,
    pub border_g: f32,
    pub border_b: f32,
    pub border_a: f32,

    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur: f32,
    pub shadow_r: f32,
    pub shadow_g: f32,
    pub shadow_b: f32,
    pub shadow_a: f32,

    pub clip_start: u32,
    pub clip_count: u32,
}

#[derive(encase::ShaderType, Default)]
pub struct ClipShape {
    pub x: f32,
    pub y: f32,
    pub half_width: f32,
    pub half_height: f32,
    pub radius: f32,
}

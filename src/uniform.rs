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

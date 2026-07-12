use wgpu_text::{BrushBuilder, TextBrush, glyph_brush::ab_glyph::FontArc};
use winit::dpi::PhysicalSize;

pub struct TextPipeline {
    pub brush: TextBrush,
}
impl TextPipeline {
    pub fn default_pipeline(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        size: PhysicalSize<u32>,
    ) -> Self {
        let font = FontArc::try_from_vec(include_bytes!("../../DejaVuSans.ttf").to_vec()).unwrap();
        let brush = BrushBuilder::using_font(font).build(
            device,
            size.width.max(1),
            size.height.max(1),
            surface_format,
        );
        Self { brush }
    }
}

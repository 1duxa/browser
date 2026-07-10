use wgpu_text::glyph_brush::Section;

use crate::uniform::ShaderSquare;

pub enum Component<'a> {
    Square(ShaderSquare),
    Text(Section<'a>),
}

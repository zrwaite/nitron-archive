use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Clone)]
pub struct UIStyles {
	pub color: Color,
	pub dimensions: Rect,
	pub padding: u32,
	pub border_color: Color,
}
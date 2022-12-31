use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::entity_lib::{Entity, EntityType};

#[derive(Clone)]
pub struct UIText {
	pub text: String,
	pub font: String,
	pub font_size: u16,
	pub font_color: Color,
}
impl UIText {
	pub fn new(text: String, font: String, font_size: u16, font_color: Color) -> Self {
		Self {
			text,
			font,
			font_size,
			font_color,
		}
	}
	pub fn default() -> Self {
		UIText::new("".to_string(), "electrolize".to_string(), 16, Color::RGBA(0, 0, 0, 0))
	}
	pub fn new_entity(
		d: Rect,
		text: String,
		font: String,
		font_size: u16,
		font_color: Color
	) -> Entity {
		Entity::new(
			d,
			EntityType::Text(UIText::new(
				text,
				font,
				font_size,
				font_color,
			))
		)
	}
}
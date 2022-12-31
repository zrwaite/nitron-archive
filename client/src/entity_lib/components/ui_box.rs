use sdl2::rect::Rect;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;
use sdl2::pixels::Color;
use crate::entity_lib::{Entity, EntityType};

#[derive(Component, Clone)]
pub struct UIBox {
	pub border_radius: i32,
	pub color: Color,
	pub padding: u32,
	pub border_color: Color
}


impl UIBox {
	pub fn new(br: i32, color: Color, bc: Color, padding: u32) -> Self {
		Self {
			border_radius: br,
			color,
			padding,
			border_color: bc
		}
	}
	pub fn default() -> Self {
		UIBox::new(0, Color::RGBA(0, 0, 0, 0), Color::RGBA(0, 0, 0, 0), 0)
	}
	pub fn new_entity(
		d: Rect,
		br: i32, 
		color: Color, 
		bc: Color, 
		padding: u32
	) -> Entity {
		Entity::new(
			d,
			EntityType::Box(UIBox::new(
				br, 
				color, 
				bc, 
				padding
			))
		)
	}
}
use sdl2::rect::Rect;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;
use crate::entity_lib::{Entity, EntityType};

#[derive(Component, Clone)]
pub struct UIImage {
	pub texture_key: String,
	pub crop: Rect
}


impl UIImage {
	pub fn new(texture: String, crop: Rect) -> Self {
		Self {
			texture_key: texture,
			crop
		}
	}
	pub fn new_entity(
		d: Rect,
		texture: String,
		crop: Rect
	) -> Entity {
		Entity::new(
			d,
			EntityType::Image(UIImage::new(
				texture,
				crop
			))
		)
	}
}
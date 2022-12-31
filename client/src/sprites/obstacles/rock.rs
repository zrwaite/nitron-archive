use sdl2::rect::Rect;

use crate::entity_lib::Entity;
use crate::utils::{Vector2, Vector3};
use crate::sprites::{StaticObstacle};

impl StaticObstacle {
	pub fn rock(pos: Vector2, size: Vector3) -> (Self, Entity) {
		Self::default_new(
			Rect::from_center((pos.x, pos.y), size.x as u32, size.y as u32), 
			size.z as u32, 
			String::from("Rock"), 
			Rect::new(80, 32, 32, 16), 
			pos, 
			Vector2::new(size.x, size.y)
		)
	}
}

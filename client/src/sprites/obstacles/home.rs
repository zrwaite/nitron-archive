use sdl2::rect::Rect;
use crate::entity_lib::Entity;
use crate::physics::Hitbox;
use crate::utils::{Vector2, Vector3};
use crate::sprites::{StaticObstacle};


impl StaticObstacle {
	pub fn home(pos: Vector2, size: Vector3) -> (Self, Entity) {
		Self::new_custom_hitbox(
			Rect::from_center((pos.x, pos.y), size.x as u32, size.y as u32), 
			size.z as u32, 
			String::from("Home"), 
			Rect::new(0, 0, 24, 24), 
			pos, 
			Vector2::new(size.x, size.y),
			Hitbox {
				w: pos.x as u32,
				h: size.z as u32,
				y_offset: pos.y / 2 - size.z / 2,
				x_offset: 0,
				radius: ((pos.x + pos.y) / 3) as u32,
			}
		)
	}
}
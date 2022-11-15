use sdl2::rect::Rect;
use crate::physics::Hitbox;
use crate::utils::{Vector2, Vector3};
use crate::entities::sprites::{StaticObstacle};
use crate::assets::TEXTURES;

pub fn generate_home(pos: Vector2, size: Vector3) -> StaticObstacle {
	StaticObstacle::new(
		pos, 
		size, 
		String::from(TEXTURES.home), 
		Hitbox {
			w: (size.x as f32 * 0.9) as u32,
			h: (size.z as f32 * 1.0) as u32,
			y_offset: size.y / 2 - (size.z as f32 * 1.2) as i32 / 2,
			x_offset: 0,
			radius: (size.x + size.y) as u32,
		},
		Rect::new(0, 0, 24, 24)
	)
}
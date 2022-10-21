use crate::components::{Vector2, Vector3, Vector4};
use crate::physics::Hitbox;
use crate::sprites::MovingSpriteDisplay;
use super::PlayerAnimator;
use sdl2::rect::Rect;

pub struct Player {
	pub display: MovingSpriteDisplay,
	pub pos: Vector2,
	pub vel: Vector2,
	pub animator: PlayerAnimator,
	hitbox: Hitbox,
	// pub stats: CharacterStats,
}

impl Player {
	pub fn new(
		pos: Vector2, 
		size: Vector3,
		texture_key: String,
	) -> Self {     
		Player {
			display: MovingSpriteDisplay::new(texture_key, size.to_vector2()),
			pos,
			vel: Vector2::new(0, 0),
			animator: PlayerAnimator::new(),
			hitbox: Hitbox {
				w: (size.x as f32 * 0.9) as u32,
				h: size.z as u32,
				y_offset: size.y / 2 - size.z / 2,
				x_offset: 0,
			},
		}
	}
	pub fn hitbox(&self) -> Vector4 {
		self.hitbox.to_v4(self.pos)
	}
	pub fn set_x_by_hitbox(&mut self, x: i32) {
		self.pos.x = x - self.hitbox.x_offset;
	}
	pub fn set_y_by_hitbox(&mut self, y: i32) {
		self.pos.y = y - self.hitbox.y_offset;
	}
}
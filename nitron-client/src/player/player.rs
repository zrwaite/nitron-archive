use crate::components::{Vector2};
use crate::sprites::MovingSpriteDisplay;
use super::PlayerAnimator;

pub struct Player {
	pub display: MovingSpriteDisplay,
	pub pos: Vector2,
	pub vel: Vector2,
	pub animator: PlayerAnimator,
	pub hitbox: Vector2,
	// pub stats: CharacterStats,
}

impl Player {
	pub fn new(
		pos: Vector2, 
		size: Vector2,
		texture_key: String,
	) -> Self {     
		Player {
			display: MovingSpriteDisplay::new(texture_key, size),
			pos,
			vel: Vector2::new(0, 0),
			animator: PlayerAnimator::new(),
			hitbox: size.get_scaled(0.5),
		}
	}
}
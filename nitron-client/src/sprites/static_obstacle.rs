use sdl2::rect::Rect;

use crate::{components::Vector2, animation::AnimationFrame};

use super::SpriteDisplay;

pub struct StaticObstacle {
	pub display: SpriteDisplay,
	pub pos: Vector2,
	pub hitbox: Vector2,
	pub frame: AnimationFrame
}

impl StaticObstacle {
	pub fn new(
		pos: Vector2, 
		size: Vector2,
		texture_key: String,
		frame_region: Rect
	) -> Self {     
		StaticObstacle {
			display: SpriteDisplay::new(texture_key, size),
			pos,
			hitbox: size.get_scaled(0.5),
			frame: AnimationFrame::new(frame_region)
		}
	}
}
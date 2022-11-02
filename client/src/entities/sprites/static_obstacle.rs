use sdl2::rect::Rect;

use crate::components::{Vector2, Vector3, Vector4};
use crate::animation::AnimationFrame;
use crate::physics::Hitbox;

use super::SpriteDisplay;

#[derive(Debug)]
pub struct StaticObstacle {
	pub display: SpriteDisplay,
	pub pos: Vector2,
	hitbox: Hitbox,
	pub frame: AnimationFrame
}

impl StaticObstacle {
	pub fn new(
		pos: Vector2, 
		size: Vector3,
		texture_key: String,
		frame_region: Rect
	) -> Self {     
		StaticObstacle {
			display: SpriteDisplay::new(texture_key, size.to_vector2()),
			pos,
			hitbox: Hitbox {
				w: (size.x as f32 * 0.7) as u32,
				h: (size.z as f32 * 0.8) as u32,
				y_offset: size.y / 2 - size.z / 2,
				x_offset: 0,
			},
			frame: AnimationFrame::new(frame_region)
		}
	}
	pub fn hitbox(&self) -> Vector4 {
		self.hitbox.to_v4(self.pos)
	}
}
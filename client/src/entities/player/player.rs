use sdl2::rect::Rect;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::utils::{Vector2, Vector3, Vector4};
use crate::entities::HasId;
use crate::physics::{Hitbox, InteractionHitbox};
use crate::entities::MovingSpriteDisplay;
use crate::utils::new_id;

use super::PlayerAnimator;

#[derive(Component, Clone)]
pub struct Player {
	id: String,
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
			id: new_id(),
			display: MovingSpriteDisplay::new(texture_key, size.to_vector2()),
			pos,
			vel: Vector2::new(0, 0),
			animator: PlayerAnimator::new(),
			hitbox: Hitbox {
				w: (size.x as f32 * 0.6) as u32,
				h: size.z as u32,
				y_offset: size.y / 2 - size.z / 2,
				x_offset: 0,
				radius: (size.x + size.y) as u32,
			},
		}
	}
	pub fn hitbox(&self) -> Vector4 {
		self.hitbox.to_v4(self.pos)
	}
	pub fn interaction_hitbox(&self) -> InteractionHitbox {
		InteractionHitbox::from_hitbox(&self.hitbox, self.pos)
	}
	pub fn set_x_by_hitbox(&mut self, x: i32) {
		self.pos.x = x - self.hitbox.x_offset;
	}
	pub fn set_y_by_hitbox(&mut self, y: i32) {
		self.pos.y = y - self.hitbox.y_offset;
	}
	pub fn contains_point(&self, x: i32, y: i32) -> bool {
		let rect = Rect::new(
			self.pos.x as i32,
			self.pos.y as i32,
			self.display.size.x as u32,
			self.display.size.y as u32,
		);
		rect.contains_point((x, y))
	}
}
impl HasId for Player {
	fn id(&self) -> String {
		self.id.clone()
	}
}
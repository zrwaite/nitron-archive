use sdl2::rect::Rect;
use crate::utils::{Vector2, Vector3, Vector4};
use crate::entities::HasId;
use crate::physics::{Hitbox, InteractionHitbox};
use crate::utils::new_id;

use super::super::SpriteDisplay;

#[derive(Debug, Clone)]
pub struct StaticObstacle {
	id: String,
	pub display: SpriteDisplay,
	pub pos: Vector2,
	hitbox: Hitbox,
	pub frame: Rect,
	pub player_interaction: bool,
}

impl StaticObstacle {
	pub fn new(
		pos: Vector2, 
		size: Vector3,
		texture_key: String,
		hitbox: Hitbox,
		frame_region: Rect
	) -> Self {     
		StaticObstacle {
			id: new_id(),
			display: SpriteDisplay::new(texture_key, size.to_vector2()),
			pos,
			hitbox,
			frame: frame_region,
			player_interaction: false,
		}
	}
	pub fn default_new(
		pos: Vector2, 
		size: Vector3,
		texture_key: String,
		frame_region: Rect
	) -> Self {    
		StaticObstacle::new(
			pos, 
			size, 
			texture_key, 
			Hitbox {
				w: size.x as u32,
				h: size.z as u32,
				y_offset: size.y / 2 - size.z / 2,
				x_offset: 0,
				radius: (size.x + size.y) as u32,
			},
			frame_region
		)
	}
	pub fn hitbox(&self) -> Vector4 {
		self.hitbox.to_v4(self.pos)
	}
	pub fn interaction_hitbox(&self) -> InteractionHitbox {
		InteractionHitbox::from_hitbox(&self.hitbox, self.pos)
	}
	pub fn contains_point(&self, x:i32, y:i32) -> bool {
		let rect = Rect::new(
			self.pos.x as i32,
			self.pos.y as i32,
			self.display.size.x as u32,
			self.display.size.y as u32,
		);
		rect.contains_point((x, y))
	}
	pub fn enable_player_interaction(&mut self) {
		self.player_interaction = true;
	}
	pub fn disable_player_interaction(&mut self) {
		self.player_interaction = false;
	}
}

impl HasId for StaticObstacle {
	fn id(&self) -> String {
		self.id.clone()
	}
}
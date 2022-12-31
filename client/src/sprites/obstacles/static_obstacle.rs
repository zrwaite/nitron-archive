use sdl2::rect::Rect;
use crate::entity_lib::{Entity, UIImage};
use crate::utils::{Vector4, Vector2};
use crate::physics::{Hitbox, InteractionHitbox};
use crate::utils::new_id;

#[derive(Debug, Clone)]
pub struct StaticObstacle {
	id: String,
	entity_id: String,
	hitbox: Hitbox,
	pub player_interaction: bool,
	pub game_pos: Vector2,
	pub game_size: Vector2,
}

impl StaticObstacle {
	pub fn new(
		hitbox: Hitbox,
		entity_id: String,
		game_pos: Vector2,
		game_size: Vector2,
	) -> Self {  
		Self {
			id: new_id(),
			entity_id,
			hitbox,
			player_interaction: false,
			game_pos,
			game_size
		}
	}
	pub fn entity(&self) -> String {
		self.entity_id.clone()
	}
	pub fn new_custom_hitbox(
		d: Rect,
		z: u32,
		texture_key: String,
		frame_region: Rect,
		game_pos: Vector2,
		game_size: Vector2,
		hitbox: Hitbox,
	) -> (Self, Entity) {
		let entity = UIImage::new_entity(
			d, 
			texture_key.clone(),
			frame_region
		);
		let entity_id = entity.id();
		(
			StaticObstacle::new(
				hitbox,
				entity_id,
				game_pos,
				game_size
			),
			entity
		)
	}
	pub fn default_new(
		d: Rect,
		z: u32,
		texture_key: String,
		frame_region: Rect,
		game_pos: Vector2,
		game_size: Vector2,
	) -> (Self, Entity) {
		Self::new_custom_hitbox(d, z, texture_key, frame_region, game_pos, game_size, Hitbox {
			w: d.x as u32,
			h: z,
			y_offset: d.y / 2 - z as i32 / 2,
			x_offset: 0,
			radius: ((d.x + d.y) / 3) as u32,
		})
	}
	pub fn hitbox(&self) -> Vector4 {
		self.hitbox.to_v4(self.game_pos)
	}
	pub fn interaction_hitbox(&self) -> InteractionHitbox {
		InteractionHitbox::from_hitbox(&self.hitbox, self.game_pos)
	}
	pub fn contains_point(&self, x:i32, y:i32) -> bool {
		let rect = Rect::new(
			self.game_pos.x as i32,
			self.game_pos.y as i32,
			self.game_size.x as u32,
			self.game_size.y as u32,
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
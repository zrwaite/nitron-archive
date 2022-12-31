use sdl2::rect::Rect;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::entity_lib::UIImage;
use crate::sprites::PlayerInteraction;
use crate::entity_lib::Entity;
use crate::utils::{Vector2, Vector3, Vector4};
use crate::physics::{Hitbox, InteractionHitbox};
use crate::sprites::MovingSpriteDisplay;
use crate::utils::new_id;

use super::NpcAnimator;

#[derive(Component, Clone)]
pub struct Npc {
	pub entity_id: String,
	pub vel: Vector2,
	pub game_pos: Vector2,
	pub game_size: Vector2,
	hitbox: Hitbox,
	player_interaction: PlayerInteraction
	// pub display: MovingSpriteDisplay,
	// pub animator: NpcAnimator,
	// pub stats: CharacterStats,
}

impl Npc {
	pub fn new(
		pos: Vector2, 
		size: Vector3,
		texture_key: String,
		frame_region: Rect
	) -> (Self, Vec<Entity>) {     
		let (player_interaction, player_interaction_ui) = PlayerInteraction::new(Vector2::new(pos.x, pos.y - size.y/2));
		let entity = UIImage::new_entity(
			Rect::from_center((pos.x, pos.y), size.x as u32, size.y as u32), 
			texture_key.clone(),
			frame_region
		);
		let entity_id = entity.id();
		(
			Npc {
				// display: MovingSpriteDisplay::new(texture_key, size.to_vector2()),
				game_pos: pos,
				game_size: Vector2::new(size.x, size.y),
				entity_id,
				vel: Vector2::new(0, 0),
				// animator: NpcAnimator::new(),
				hitbox: Hitbox {
					w: (size.x as f32 * 0.6) as u32,
					h: size.z as u32,
					y_offset: size.y / 2 - size.z / 2,
					x_offset: 0,
					radius: ((size.x + size.y) / 3) as u32,
				},
				player_interaction
			},
			player_interaction_ui
		)
	}
	pub fn hitbox(&self) -> Vector4 {
		self.hitbox.to_v4(self.game_pos)
	}
	pub fn interaction_hitbox(&self) -> InteractionHitbox {
		InteractionHitbox::from_hitbox(&self.hitbox, self.game_pos)
	}
	pub fn set_x_by_hitbox(&mut self, x: i32) {
		self.game_pos.x = x - self.hitbox.x_offset;
	}
	pub fn set_y_by_hitbox(&mut self, y: i32) {
		self.game_pos.y = y - self.hitbox.y_offset;
	}
	pub fn contains_point(&self, x: i32, y: i32) -> bool {
		let rect = Rect::new(
			self.game_pos.x as i32,
			self.game_pos.y as i32,
			self.game_size.x as u32,
			self.game_size.y as u32,
		);
		rect.contains_point((x, y))
	}
	// pub fn enable_player_interaction(&mut self, ui_boxes: &mut HashMap<String, &mut UIBox>) {
	// 	self.player_interaction.on(ui_boxes);
	// }
	// pub fn disable_player_interaction(&mut self, ui_boxes: &mut HashMap<String, &mut UIBox>) {
	// 	self.player_interaction.off(ui_boxes);
	// }
}
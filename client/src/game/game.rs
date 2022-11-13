use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::assets::TEXTURES;
use crate::entities::GameEntity;
use crate::entities::Player;
use crate::entities::HasId;
use crate::utils::{Vector2, Vector3};

use super::BlockMap;
use super::load_chunk;


#[derive(Component)]
pub struct Game {
	pub player_id: String,
	pub blocks: Vec<BlockMap>,
	block_index: usize,
}

impl Game {
	pub fn new()-> (Self, Vec<GameEntity>) {
		let player = Player::new(Vector2::new(100, 100), Vector3::new(32, 40, 10), String::from(TEXTURES.player));

		let (blocks, map_entities) = load_chunk("nitron_city".to_string()).unwrap();
		let mut entities = vec![
			GameEntity::Player(player),
		];
		entities.append(&mut map_entities.to_vec());

		(
			Self {
				player_id: entities[0].id(),
				block_index: 0,
				blocks,
			},
			entities
		)
	}
	pub fn block(&self) -> &BlockMap {
		&self.blocks[self.block_index]
	}
}
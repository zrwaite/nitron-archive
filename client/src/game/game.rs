use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::assets::TEXTURES;
use crate::entities::GameEntity;
use crate::entities::Player;
use crate::entities::HasId;
use crate::entities::sprites::{rock::generate_rock};
use crate::utils::{Vector2, Vector3};


#[derive(Component)]
pub struct Game {
	pub player_id: String,
	pub map: GameMap,
}

impl Game {
	pub fn new(width: u32, height: u32)-> (Self, Vec<GameEntity>) {
		let player = Player::new(Vector2::new(100, 100), Vector3::new(32, 40, 10), String::from(TEXTURES.player));

		let (map, map_entities) = GameMap::new(width, height);

		let mut entities = vec![
			GameEntity::Player(player),
		];
		entities.append(&mut map_entities.to_vec());

		(
			Self {
				player_id: entities[0].id(),
				map,
			},
			entities
		)
	}
}

pub struct GameMap {
	pub width: u32,
	pub height: u32,
	pub static_obstacle_ids: Vec<String>,
}

impl GameMap {
	pub fn new(width: u32, height: u32) -> (Self, Vec<GameEntity>) {
		let static_obstacles = Vec::from(
			[generate_rock(Vector2::new(200, 200), Vector3::new(40, 20, 15))]
		);
		let static_obstacle_ids = static_obstacles.iter().map(|entity| entity.id()).collect();
		let entities = static_obstacles.into_iter().map(|entity| GameEntity::StaticObstacle(entity)).collect();
		(
			Self {
				width,
				height,
				static_obstacle_ids,
			},
			entities
		)
	}
}
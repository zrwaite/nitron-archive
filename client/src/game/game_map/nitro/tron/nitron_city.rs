use crate::{game::BlockMap, entities::{GameEntity, generate_rock}, utils::{Vector2, Vector3}};
use crate::entities::hash_vec::HasId;

pub fn nitron_city() -> (Vec<BlockMap>, Vec<GameEntity>) {
	let mut entities = vec![];
	let mut blocks = vec![];
	// let mut block_slugs = vec![];

	let (block, block_entities) = home_block_map();
	blocks.push(block);
	entities.append(&mut block_entities.to_vec());

	(
		blocks,
		entities
	)
}

pub fn home_block_map() -> (BlockMap, Vec<GameEntity>) {
	let static_obstacles = Vec::from(
		[generate_rock(Vector2::new(200, 200), Vector3::new(40, 20, 15))]
	);
	let static_obstacle_ids = static_obstacles.iter().map(|entity| entity.id()).collect();
	let entities = static_obstacles.into_iter().map(|entity| GameEntity::StaticObstacle(entity)).collect();
	(
		BlockMap {
			slug: String::from("home"),
			width: 400,
			height: 300,
			static_obstacle_ids,
		},
		entities
	)
}
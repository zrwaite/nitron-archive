use crate::{game::BlockMap, entities::{GameEntity, generate_rock, generate_home, generate_steve}, utils::{Vector2, Vector3}};
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
	let static_obstacles = Vec::from([
		generate_rock(Vector2::new(200, 200), Vector3::new(40, 20, 15)),
		generate_home(Vector2::new(300, 200), Vector3::new(100, 100, 30))
	]);
	let static_obstacle_ids = static_obstacles.iter().map(|entity| entity.id()).collect();
	let npcs = Vec::from([
		generate_steve(Vector2::new(100, 100))
	]);
	let npc_ids = npcs.iter().map(|entity| entity.id()).collect();

	let mut entities: Vec<GameEntity> = vec![];

	let static_obstacles_entities: Vec<GameEntity> = static_obstacles.into_iter().map(|entity| GameEntity::StaticObstacle(entity)).collect();
	let npcs_entities: Vec<GameEntity> = npcs.into_iter().map(|entity| GameEntity::Npc(entity)).collect();

	entities.append(&mut static_obstacles_entities.to_vec());
	entities.append(&mut npcs_entities.to_vec());
	(
		BlockMap {
			slug: String::from("home"),
			width: 400,
			height: 300,
			static_obstacle_ids,
			npc_ids,
		},
		entities
	)
}
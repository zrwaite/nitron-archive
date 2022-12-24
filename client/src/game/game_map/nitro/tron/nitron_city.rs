use crate::{game::BlockMap};
use crate::entity_lib::Entity;
use crate::sprites::{StaticObstacle, Npc};
use crate::utils::{Vector2, Vector3};

pub fn nitron_city() -> (Vec<BlockMap>, Vec<Entity>) {
	let mut entities: Vec<Entity> = vec![];
	let mut blocks = vec![];
	// let mut block_slugs = vec![];

	let (block, mut block_entities) = home_block_map();
	blocks.push(block);
	entities.append(&mut block_entities);

	(
		blocks,
		entities
	)
}

pub fn home_block_map() -> (BlockMap, Vec<Entity>) {
	let static_obstacles = Vec::from([
		StaticObstacle::rock(Vector2::new(200, 200), Vector3::new(40, 20, 15)),
		StaticObstacle::home(Vector2::new(300, 200), Vector3::new(100, 100, 30))
	]);
	let static_obstacle_ids = static_obstacles.iter().map(|obstacle| obstacle.0.entity()).collect();
	let (steve, mut steve_entities) = Npc::steve(Vector2::new(100, 200));
	let npcs = Vec::from([
		steve
	]);
	let npc_ids = npcs.iter().map(|npc| npc.entity_id.clone()).collect();

	let mut entities: Vec<Entity> = vec![];

	let mut static_obstacles_entities: Vec<Entity> = static_obstacles.into_iter().map(|obstacle| obstacle.1).collect();

	entities.append(&mut static_obstacles_entities);
	entities.append(&mut steve_entities);

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
use specs_derive::Component;
use specs::prelude::{Component, VecStorage};


use crate::{player::Player, game_map::GameMap};

// Component
#[derive(Component)]
#[storage(VecStorage)]
pub struct Game {
	pub map: GameMap,
	pub player: Player,
}

impl Game {
	pub fn new(
		game_width: u32, 
		game_height: u32,
		player: Player,
	) -> Self {
		Self {
			map: GameMap::new(game_width, game_height),
			player,
		}
	}
}
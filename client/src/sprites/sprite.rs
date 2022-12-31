use crate::sprites::{Player, StaticObstacle};

use super::{Npc};

#[derive(Clone)]
pub enum Sprite {
	Player(Player),
	StaticObstacle(StaticObstacle),
	Npc(Npc),
}

impl Sprite {
	pub fn mut_unwrap_player(&mut self) -> &mut Player {
		match self {
			Sprite::Player(player) => player,
			_ => panic!("expected player"),
		}
	}
	pub fn enabled(&self) -> bool {
		match self {
			Sprite::Player(_player) => true,
			Sprite::StaticObstacle(_obstacle) => true,
			Sprite::Npc(_npc) => true,
		}
	}
}

// impl HasZIndex for Sprite {
// 	fn z_index(&self) -> i32 {
// 		match self {
// 			Sprite::Player(player) => player.hitbox().y,
// 			Sprite::StaticObstacle(obstacle) => obstacle.hitbox().y,
// 			Sprite::Npc(npc) => npc.hitbox().y
// 		}
// 	}
// }
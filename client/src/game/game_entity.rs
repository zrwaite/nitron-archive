use std::collections::HashMap;

use sdl2::{render::{WindowCanvas, Texture}, ttf::Font};

use crate::{graphics::{Renderable}, models::GetId, input::MouseActions, ui::UIBox, events::EngineEvent, entities::{Player, StaticObstacle}};

#[derive(Clone)]
pub enum GameEntity {
	Box(UIBox),
	Player(Player),
	StaticObstacle(StaticObstacle)
}

impl GameEntity {
	pub fn mut_unwrap_player(&mut self) -> &mut Player {
		match self {
			GameEntity::Player(player) => player,
			_ => panic!("expected player"),
		}
	}
}

impl Renderable for GameEntity {
	fn render(&self, 
		canvas: &mut WindowCanvas,
		textures: &HashMap<String, Texture>,
    	fonts: &HashMap<String, Font>,
		x_scale: f64,
		y_scale: f64,
		debug: bool
	) {
		match self {
			GameEntity::Box(box_) => box_.render(canvas, textures, fonts, x_scale, y_scale, debug),
			GameEntity::Player(player) => player.render(canvas, textures, fonts, x_scale, y_scale, debug),
			GameEntity::StaticObstacle(obstacle) => obstacle.render(canvas, textures, fonts, x_scale, y_scale, debug)
		}
	}
}

impl GetId for GameEntity {
	fn get_id(&self) -> String {
		match self {
			GameEntity::Box(box_) => box_.get_id(),
			GameEntity::Player(player) => player.get_id(),
			GameEntity::StaticObstacle(obstacle) => obstacle.get_id()
		}
	}
}

impl MouseActions for GameEntity {
	fn contains_point(&self, x: i32, y: i32) -> bool {
		match self {
			GameEntity::Box(box_) => box_.contains_point(x, y),
			GameEntity::Player(player) => player.contains_point(x, y),
			GameEntity::StaticObstacle(obstacle) => obstacle.contains_point(x, y)
		}
	}
	fn mouse_down(&mut self, x: i32, y: i32) -> Option<EngineEvent>{
		match self {
			GameEntity::Box(box_) => box_.mouse_down(x, y),
			GameEntity::Player(_player) => None,
			GameEntity::StaticObstacle(_obstacle) => None
		}
	}

	fn mouse_up(&mut self, x: i32, y: i32) -> Option<EngineEvent>{
		match self {
			GameEntity::Box(box_) => box_.mouse_up(x, y),
			GameEntity::Player(_player) => None,
			GameEntity::StaticObstacle(_obstacle) => None
		}
	}

	fn mouse_move(&mut self, x: i32, y: i32) -> Option<EngineEvent>{
		match self {
			GameEntity::Box(box_) => box_.mouse_move(x, y),
			GameEntity::Player(_player) => None,
			GameEntity::StaticObstacle(_obstacle) => None
		}
	}
}
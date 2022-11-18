use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::ttf::Font;

use crate::graphics::{Renderable, HasZIndex, simple_render};
use crate::input::MouseActions;
use crate::physics::InteractionHitbox;
use crate::ui::UIBox;
use crate::entities::{Player, StaticObstacle};
use crate::engine::EngineEvent;
use crate::utils::{Vector2, Vector4};
use super::{HasId, Npc};

#[derive(Clone)]
pub enum GameEntity {
	Box(UIBox),
	Player(Player),
	StaticObstacle(StaticObstacle),
	Npc(Npc),
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
		let pos: Vector2;
		let texture_key: String;
		let display_size: Vector2;
		let frame: Rect;
		let hitbox: Vector4;
		let interaction_hitbox: InteractionHitbox;
		match self {
			GameEntity::Box(box_) => {
				box_.render(canvas, textures, fonts, x_scale, y_scale, debug);
				return
			}
			GameEntity::Player(player) => {
				pos = player.pos;
				texture_key = player.display.texture_key.clone();
				display_size = player.display.size;
				frame = player.animator.current_frame;
				hitbox = player.hitbox();
				interaction_hitbox = player.interaction_hitbox();
			}
			GameEntity::StaticObstacle(obstacle) => {
				pos = obstacle.pos;
				texture_key = obstacle.display.texture_key.clone();
				display_size = obstacle.display.size;
				frame = obstacle.frame;
				hitbox = obstacle.hitbox();
				interaction_hitbox = obstacle.interaction_hitbox()
			}
			GameEntity::Npc(npc) => {
				pos = npc.pos;
				texture_key = npc.display.texture_key.clone();
				display_size = npc.display.size;
				frame = npc.animator.current_frame;
				hitbox = npc.hitbox();
				interaction_hitbox = npc.interaction_hitbox()
			}
		}
		simple_render(canvas, pos, texture_key, display_size, frame, hitbox, interaction_hitbox, textures, fonts, x_scale, y_scale, debug)
	}
}

impl HasId for GameEntity {
	fn id(&self) -> String {
		match self {
			GameEntity::Box(box_) => box_.id(),
			GameEntity::Player(player) => player.id(),
			GameEntity::StaticObstacle(obstacle) => obstacle.id(),
			GameEntity::Npc(npc) => npc.id()
		}
	}
}

impl MouseActions for GameEntity {
	fn contains_point(&self, x: i32, y: i32) -> bool {
		match self {
			GameEntity::Box(box_) => box_.contains_point(x, y),
			GameEntity::Player(player) => player.contains_point(x, y),
			GameEntity::StaticObstacle(obstacle) => obstacle.contains_point(x, y),
			GameEntity::Npc(npc) => npc.contains_point(x, y)
		}
	}
	fn mouse_down(&mut self, x: i32, y: i32) -> Option<EngineEvent>{
		match self {
			GameEntity::Box(box_) => box_.mouse_down(x, y),
			GameEntity::Player(_player) => None,
			GameEntity::StaticObstacle(_obstacle) => None,
			GameEntity::Npc(_npc) => None
		}
	}

	fn mouse_up(&mut self, x: i32, y: i32) -> Option<EngineEvent>{
		match self {
			GameEntity::Box(box_) => box_.mouse_up(x, y),
			GameEntity::Player(_player) => None,
			GameEntity::StaticObstacle(_obstacle) => None,
			GameEntity::Npc(_npc) => None
		}
	}

	fn mouse_move(&mut self, x: i32, y: i32) -> Option<EngineEvent>{
		match self {
			GameEntity::Box(box_) => box_.mouse_move(x, y),
			GameEntity::Player(_player) => None,
			GameEntity::StaticObstacle(_obstacle) => None,
			GameEntity::Npc(_npc) => None
		}
	}
}

impl HasZIndex for GameEntity {
	fn z_index(&self) -> i32 {
		match self {
			GameEntity::Box(box_) => box_.z_index,
			GameEntity::Player(player) => player.hitbox().y,
			GameEntity::StaticObstacle(obstacle) => obstacle.hitbox().y,
			GameEntity::Npc(npc) => npc.hitbox().y
		}
	}
}
use sdl2::rect::Rect;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;
use sdl2::pixels::Color;

use crate::GAME_HEIGHT;
use crate::GAME_WIDTH;
use crate::data::GameData;
use crate::engine::EngineFn;
use crate::sprites::Player;
use crate::entity_lib::Entity;
use crate::entity_lib::EntityStore;
use crate::entity_lib::UIBox;
use crate::ui_components::create_pause_menu;
use crate::ui_components::create_text_button;
use crate::utils::{Vector2, Vector3};

use super::BlockMap;
use super::load_chunk;


#[derive(Component)]
pub struct Game {
	pub player: Player,
	pub chunk_slug: String,
	pub blocks: Vec<BlockMap>,
	pub pause_menu_id: String,
	pub game_screen_id: String,
	block_index: usize,
}

impl Game {
	pub fn new()-> (Self, Vec<Entity>) {
		Game::from(GameData {
			chunk_slug: "nitron_city".to_string(),
			block_index: 0,
			pos: Vector2::new(200, 150),
		})
	}
	pub fn block(&self) -> &BlockMap {
		&self.blocks[self.block_index]
	}
	pub fn from(game_data: GameData)-> (Self, Vec<Entity>) {
		let player = Player::new(
			game_data.pos, 
			Vector3::new(32, 40, 10), 
			String::from("player")
		);


		let (pause_menu_boxes, pause_menu_id) = create_pause_menu();
		// let mut pause_menu_box_entities: Vec<Entity> = pause_menu_boxes.into_iter().map(|box_| Entity::Box(box_)).collect();

		let (blocks, mut map_entities) = load_chunk(game_data.chunk_slug.clone()).unwrap();
		let (pause_button, mut pause_button_children) = create_text_button(
			Rect::from_center((20, 20), 30, 30),
			Color::RGB(0, 200, 250),
			"||".to_string(),
			Some(EngineFn::new(|engine| {
				engine.pause();
				println!("pause");
			})),
		);
		let mut game_screen = UIBox::new_entity(
			Rect::new(0, 0, GAME_WIDTH, GAME_HEIGHT),
			0,
			Color::RGBA(0, 0, 0, 0),
			Color::RGBA(0, 0, 0, 0),
			0
		);
		game_screen.append_child(pause_button.id());
		let game_screen_id = game_screen.id();
		let mut entities = vec![
			pause_button,
			game_screen,
		];
		entities.append(&mut map_entities);
		entities.append(&mut pause_button_children);

		(
			Self {
				player,
				block_index: game_data.block_index,
				blocks,
				pause_menu_id,
				game_screen_id,
				chunk_slug: game_data.chunk_slug,
			},
			entities
		)
	}
	pub fn to_game_data(&self, entity_store: &mut EntityStore) -> GameData {
		GameData {
			chunk_slug: self.chunk_slug.clone(),
			block_index: self.block_index,
			pos: self.player.pos,
		}
	}
}
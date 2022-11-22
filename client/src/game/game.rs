use sdl2::rect::Rect;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;
use sdl2::pixels::Color;

use crate::GAME_HEIGHT;
use crate::GAME_WIDTH;
use crate::assets::TEXTURES;
use crate::data::GameData;
use crate::engine::EngineFn;
use crate::entities::GameEntity;
use crate::entities::HashVec;
use crate::entities::Player;
use crate::entities::HasId;
use crate::ui::UIBox;
use crate::ui::create_pause_menu;
use crate::ui::create_text_button;
use crate::utils::{Vector2, Vector3};
use crate::ui::styles::UIStyles;

use super::BlockMap;
use super::load_chunk;


#[derive(Component)]
pub struct Game {
	pub player_id: String,
	pub chunk_slug: String,
	pub blocks: Vec<BlockMap>,
	pub pause_menu_id: String,
	pub game_screen_id: String,
	block_index: usize,
}

impl Game {
	pub fn new()-> (Self, Vec<GameEntity>) {
		Game::from(GameData {
			chunk_slug: "nitron_city".to_string(),
			block_index: 0,
			pos: Vector2::new(200, 150),
		})
	}
	pub fn block(&self) -> &BlockMap {
		&self.blocks[self.block_index]
	}
	pub fn from(game_data: GameData)-> (Self, Vec<GameEntity>) {
		let player = Player::new(
			game_data.pos, 
			Vector3::new(32, 40, 10), 
			String::from(TEXTURES.player)
		);


		let (pause_menu_boxes, pause_menu_id) = create_pause_menu();
		let mut pause_menu_box_entities: Vec<GameEntity> = pause_menu_boxes.into_iter().map(|box_| GameEntity::Box(box_)).collect();

		let (blocks, map_entities) = load_chunk(game_data.chunk_slug.clone()).unwrap();
		let pause_button = create_text_button(
			Rect::from_center((20, 20), 30, 30),
			Color::RGB(0, 200, 250),
			"||".to_string(),
			Some(EngineFn::new(|engine| {
				engine.pause();
				println!("pause");
			})),
		);
		let game_screen = UIBox::new(
			vec![
				pause_button.id()
			],
			None,
			Rect::new(0, 0, GAME_WIDTH, GAME_HEIGHT),
			UIStyles { 
				color: Color::RGBA(0, 0, 0, 0),
				padding: 0, 
				border_color: Color::RGBA(0, 0, 0, 0), 
			},
			None,
		);
		let game_screen_id = game_screen.id();
		let mut entities = vec![
			GameEntity::Player(player),
			GameEntity::Box(pause_button),
			GameEntity::Box(game_screen),
		];
		entities.append(&mut map_entities.to_vec());
		entities.append(&mut pause_menu_box_entities);

		(
			Self {
				player_id: entities[0].id(),
				block_index: game_data.block_index,
				blocks,
				pause_menu_id,
				game_screen_id,
				chunk_slug: game_data.chunk_slug,
			},
			entities
		)
	}
	pub fn to_game_data(&self, game_entities: &mut HashVec) -> GameData {
		let player = game_entities.player(self.player_id.clone());
		GameData {
			chunk_slug: self.chunk_slug.clone(),
			block_index: self.block_index,
			pos: player.pos,
		}
	}
}
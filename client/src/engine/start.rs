use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

use crate::entity_lib::Entity;
use crate::ui_components::create_text_button;
use crate::{GAME_WIDTH, GAME_HEIGHT};
use crate::data::game_data_exists;

use super::EngineFn;

pub struct StartScreen {
	// new_game_button_id: String,
	// continue_game_button_id: Option<String>,
}

impl StartScreen {
	pub fn new() -> (Self, Vec<Entity>) {
		let (mut new_game_button, mut new_game_button_children) = create_text_button(
			Rect::from_center(Point::new(GAME_WIDTH as i32/2, GAME_HEIGHT as i32/2), 140, 40),
			Color::RGB(0, 200, 150),
			"New Game".to_string(),
			Some(EngineFn::new(|engine| engine.new_game())),
		);
		let entities: Vec<Entity>;
		if game_data_exists() {
			new_game_button.move_pos(0, -40);
			let (mut continue_game_button, mut continue_game_button_children) = create_text_button(
				Rect::from_center(Point::new(GAME_WIDTH as i32/2, GAME_HEIGHT as i32/2 + 40), 190, 40),
				Color::RGB(0, 200, 250),
				"Continue Game".to_string(),
				Some(EngineFn::new(|engine| engine.continue_game())),
			);
			entities = vec![
				new_game_button,
				continue_game_button,
			]
		} else {
			entities = vec![
				new_game_button,
			]
		}
		(
			Self {},
			entities
		)
	}
}


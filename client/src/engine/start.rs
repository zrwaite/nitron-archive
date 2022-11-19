use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

use crate::{GAME_WIDTH, GAME_HEIGHT};
use crate::data::{game_data_exists};
use crate::entities::{GameEntity,HasId};
use crate::ui::components::create_text_button;

use super::EngineFn;

pub struct StartScreen {
	new_game_button_id: String,
	continue_game_button_id: Option<String>,
}

impl StartScreen {
	pub fn new() -> (Self, Vec<GameEntity>) {
		let mut new_game_button = create_text_button(
			Rect::from_center(Point::new(GAME_WIDTH as i32/2, GAME_HEIGHT as i32/2), 140, 40),
			Color::RGB(0, 200, 150),
			"New Game".to_string(),
			Some(EngineFn::new(|engine| {
				println!("New Game");
				engine.new_game()
			})),
		);
		let new_game_button_id = new_game_button.id();
		if game_data_exists() {
			new_game_button.move_box(0, -40);
			let continue_game_button = create_text_button(
				Rect::from_center(Point::new(GAME_WIDTH as i32/2, GAME_HEIGHT as i32/2 + 40), 190, 40),
				Color::RGB(0, 200, 250),
				"Continue Game".to_string(),
				Some(EngineFn::new(|engine| {
					println!("Continue Game");
					engine.continue_game()
				})),
			);
			let continue_game_button_id = continue_game_button.id();
			return (
				Self {
					new_game_button_id,
					continue_game_button_id: Some(continue_game_button_id),
				},
				vec![
					GameEntity::Box(new_game_button),
					GameEntity::Box(continue_game_button)
				]
			)
		}
		(
			Self {
				new_game_button_id,
				continue_game_button_id: None,
			},
			vec![
				GameEntity::Box(new_game_button),
			]
		)
	}
}


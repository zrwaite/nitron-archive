use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

use crate::game::GameEntity;
use crate::models::GetId;
use crate::ui::components::create_text_button;

pub struct StartScreen {
	_start_button_id: String,
}

impl StartScreen {
	pub fn new(width: u32, height: u32) -> (Self, Vec<GameEntity>) {
		let start_button = create_text_button(
			Rect::from_center(Point::new(width as i32/2, height as i32/2), 80, 40),
			Color::RGB(0, 200, 150),
			"Start".to_string(),
		);
		let start_button_id = start_button.get_id();
		let entities = vec![
			GameEntity::Box(start_button),
		];
		(
			Self {
				_start_button_id: start_button_id,
			},
			entities
		)
	}
}


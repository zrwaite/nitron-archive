use sdl2::{rect::Rect};

use sdl2::pixels::Color;

use crate::data::write_game_data;
use crate::entities::hash_vec::HasId;
use crate::{GAME_WIDTH, GAME_HEIGHT};
use crate::engine::EngineFn;
use crate::ui::{UIBox, create_text_button};
use crate::ui::styles::UIStyles;

pub fn create_pause_menu() -> (Vec<UIBox>, String) {
	let mut play_button = create_text_button(
		Rect::from_center((20, 20), 30, 30),
		Color::RGB(0, 200, 250),
		"|>".to_string(),
		Some(EngineFn::new(|engine| {
			engine.unpause();
			println!("unpause");
		})),
	);
	play_button.set_display(false);

	let mut quit_button = create_text_button(
		Rect::from_center((100, 20), 80, 40),
		Color::RGB(0, 200, 250),
		"Quit".to_string(),
		Some(EngineFn::new(|engine| {
			engine.quit();
			println!("quit");
		})),
	);
	quit_button.set_display(false);

	let mut save_button = create_text_button(
		Rect::from_center((240, 20), 120, 40),
		Color::RGB(0, 200, 250),
		"Save Game".to_string(),
		Some(EngineFn::new(|engine| {
			let game_data = engine.state.mut_unwrap_game().to_game_data(&mut engine.game_entities);
			write_game_data(&game_data);
			println!("save");
		})),
	);
	save_button.set_display(false);

	let mut pause_menu = UIBox::new(
		vec![
			play_button.id(),
			quit_button.id(),
			save_button.id(),
		],
		None,
		UIStyles {
			dimensions: Rect::new(0, 0, GAME_WIDTH, GAME_HEIGHT),
			padding: 0,
			border_color: Color::RGBA(0, 0, 0, 0),
			color: Color::RGBA(100, 100, 100, 100),
		},
		None,
	);
	let pause_menu_id = pause_menu.id();
	pause_menu.set_display(false);
	
	(
		vec![
			pause_menu,
			play_button,
			quit_button,
			save_button,
		],
		pause_menu_id
	)
}
use sdl2::{rect::Rect};

use sdl2::pixels::Color;

use crate::engine::EngineFn;
use crate::entity_lib::{UIBox, UIText, Entity};

pub fn create_text_button(
	rect: Rect,
	color: Color,
	text: String,
	on_click: Option<EngineFn>,
) -> (Entity, Vec<Entity>) {
	let mut button = UIBox::new_entity(
		rect,
		0,
		color,
		Color::RGBA(0, 0, 0, 0),
		0,
		// on_click,
	);
	let mut button_text = UIText::new_entity(
		rect,
		"electrolize".to_string(),
		text,
		30,
		Color::RGB(255, 255, 255),
	);
	button.append_child(button_text.id().clone());
	(
		button,
		vec![
			button_text
		]
	)

}
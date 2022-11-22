use sdl2::{rect::Rect};

use sdl2::pixels::Color;

use crate::engine::EngineFn;
use crate::ui::{UIBox, TextElement};
use crate::ui::styles::UIStyles;

pub fn create_text_button(
	rect: Rect,
	color: Color,
	text: String,
	on_click: Option<EngineFn>
) -> UIBox {
	create_text_button_game_scale(rect, color, text, on_click, false)
}

pub fn create_text_button_game_scale(
	rect: Rect,
	color: Color,
	text: String,
	on_click: Option<EngineFn>,
	game_scale: bool
) -> UIBox {
	UIBox::new_game_scale(
		vec![],
		Some(TextElement::simple_new(
			text,
			30,
			Color::RGB(255, 255, 255),
			rect.center().x(),
			rect.center().y(),
			game_scale
		)),
		rect,
		UIStyles {
			padding: 0,
			border_color: Color::RGBA(0, 0, 0, 0),
			color,
		},
		on_click,
		game_scale
	)
}
use sdl2::{rect::Rect};

use sdl2::pixels::Color;

use crate::engine::EngineFn;
use crate::ui::{UIBox, TextElement};
use crate::utils::new_id;
use crate::ui::styles::UIStyles;

pub fn create_text_button(
	rect: Rect,
	color: Color,
	text: String,
	on_click: Option<EngineFn>,
) -> UIBox {
	UIBox::new(
		new_id(),
		vec![],
		Some(TextElement::simple_new(
			text,
			30,
			Color::RGB(255, 255, 255),
			rect.center().x(),
			rect.center().y(),
		)),
		UIStyles {
			dimensions: rect,
			padding: 0,
			border_color: Color::RGBA(0, 0, 0, 0),
			color,
		},
		on_click,
	)
}
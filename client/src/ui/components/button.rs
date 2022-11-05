use sdl2::{rect::Rect};

use sdl2::pixels::Color;

use crate::processor::EngineEvent;
use crate::ui::{BoxElement, UIStyles, UIElement, TextElement, MouseDetails};

pub fn create_text_button(
	rect: Rect,
	color: Color,
	on_click: EngineEvent,
	text: String,
) -> UIElement {
	UIElement::Box(BoxElement {
		elements: vec![
			TextElement::simple_new(
				text,
				30,
				Color::RGB(255, 255, 255),
				rect.center().x(),
				rect.center().y(),
			)
		],
		styles: UIStyles {
			width: rect.width(),
			height: rect.height(),
			x: rect.center().x(),
			y: rect.center().y(),
			padding: 0,
			border_color: Color::RGBA(0, 0, 0, 0),
		},
		color,
		mouse_details: MouseDetails {
			hovering: false,
			clicked: false,
			on_click,
		},
	})
}
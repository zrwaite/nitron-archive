use sdl2::{rect::Rect};

use sdl2::pixels::Color;

use crate::ui::{BoxElement, UIStyles, UIElement, TextElement, MouseDetails, UIEventFunction, new_id};

pub fn create_text_button(
	rect: Rect,
	color: Color,
	on_click: UIEventFunction,
	text: String,
) -> UIElement {
	let new_color = color.clone();
	UIElement::Box(BoxElement {
		id: new_id(),
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
			on_hover: UIEventFunction::None,
			off_hover: UIEventFunction::None,
		},
	})
}
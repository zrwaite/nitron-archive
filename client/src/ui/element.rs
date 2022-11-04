use sdl2::{pixels::Color, rect::Rect};

use crate::assets::FONTS;

pub struct UIStyles {
	pub width: u32,
	pub height: u32,
	pub x: i32,
	pub y: i32,
	pub padding: u32,
}

pub struct ClickActions {
	pub enabled: bool,
	pub on_click: fn(),
	pub on_hover: fn(),
}

pub struct TextElement {
	pub text: String,
	pub font: String,
	pub font_size: u16,
	pub font_color: Color,
	pub styles: UIStyles,
}
impl TextElement {
	pub fn new_normal(text: String, font_size: u16, font_color: Color, x: i32, y: i32) -> Self {
		Self {
			text,
			font: FONTS.electrolize.to_string(),
			font_size,
			font_color,
			styles: UIStyles {
				width: 0,
				height: font_size as u32,
				x,
				y,
				padding: 0,
			},
		}
	}
}

pub struct ImageElement {
	pub image: String,
	pub styles: UIStyles,
}

pub struct BoxElement {
	pub elements: Vec<UIElement>,
	pub rect: Rect,
	pub styles: UIStyles,
	pub click_actions: ClickActions,
	pub color: Color,
}

pub enum UIElement {
	Text(TextElement),
	Image(ImageElement),
	Box(BoxElement),
}


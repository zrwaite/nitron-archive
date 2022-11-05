use sdl2::{pixels::Color, rect::Rect};

use crate::{assets::FONTS, processor::EngineEvent, graphics::{scale, scale_u}};

pub struct UIStyles {
	pub width: u32,
	pub height: u32,
	pub x: i32,
	pub y: i32,
	pub padding: u32,
}

pub struct MouseDetails {
	pub hovering: bool,
	pub clicked: bool,
	pub on_click: EngineEvent,
}

pub struct TextElement {
	pub text: String,
	pub font: String,
	pub font_size: u16,
	pub font_color: Color,
	pub styles: UIStyles,
}
impl TextElement {
	pub fn simple_new(text: String, font_size: u16, font_color: Color, x: i32, y: i32) -> Self {
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
	pub styles: UIStyles,
	pub mouse_details: MouseDetails,
	pub color: Color,
}


impl BoxElement {
	pub fn simple_new(elements: Vec<UIElement>, rect: Rect, color: Color, on_click: EngineEvent) -> BoxElement {
		BoxElement {
			elements,
			styles: UIStyles {
				width: rect.width(),
				height: rect.height(),
				x: rect.x(),
				y: rect.y(),
				padding: 0,
			},
			color,
			mouse_details: MouseDetails {
				hovering: false,
				clicked: false,
				on_click,
			},
		}
	}
	pub fn new(
		elements: Vec<UIElement>, 
		rect: Rect, 
		color: Color, 
		on_click: EngineEvent,
	) -> BoxElement {
		BoxElement {
			elements,
			styles: UIStyles {
				width: rect.width(),
				height: rect.height(),
				x: rect.x(),
				y: rect.y(),
				padding: 0,
			},
			color,
			mouse_details: MouseDetails {
				hovering: false,
				clicked: false,
				on_click,
			},
		}
	}
	pub fn get_scaled_rect(&self, x_scale: f64, y_scale: f64) -> Rect {
		Rect::new(
			scale(self.styles.x, x_scale), 
			scale(self.styles.y, y_scale),
			scale_u(self.styles.width as i32, x_scale),
			scale_u(self.styles.height as i32, y_scale),
		)
	}
	pub fn get_rect(&self) -> Rect {
		Rect::new(
			self.styles.x, 
			self.styles.y,
			self.styles.width,
			self.styles.height,
		)
	}
	pub fn contains_point(&self, x: i32, y: i32) -> bool {
		self.get_rect().contains_point((x, y))
	}

}

pub enum UIElement {
	Text(TextElement),
	Image(ImageElement),
	Box(BoxElement),
}
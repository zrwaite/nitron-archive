use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::ttf::Font;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::graphics::{scale, scale_u, Renderable};
use crate::ui::{styles::UIStyles};
use crate::input::{MouseActions, MouseDetails};
use crate::entities::HasId;
use crate::engine::{EngineEvent, EngineFn};

use super::text_element::TextElement;

#[derive(Component, Clone)]
pub struct UIBox {
	id: String,
	pub children: Vec<u32>,
	pub text_node: Option<TextElement>,
	pub styles: UIStyles,
	pub initial_styles: UIStyles,
	pub mouse_details: MouseDetails,
	pub z_index: i32,
	on_click: Option<EngineFn>,
}


impl UIBox {
	pub fn new(
		id: String,
		children: Vec<u32>,
		text_node: Option<TextElement>,
		styles: UIStyles,
		on_click: Option<EngineFn>,
	) -> Self {
		Self {
			id,
			children,
			text_node,
			styles: styles.clone(),
			initial_styles: styles,
			mouse_details: MouseDetails::new(),
			z_index: 10000,
			on_click,
		}
	}
	pub fn get_scaled_rect(&self, x_scale: f64, y_scale: f64) -> Rect {
		Rect::from_center((
				scale(self.styles.dimensions.center().x, x_scale), 
				scale(self.styles.dimensions.center().y, y_scale),
			),
			scale_u(self.styles.dimensions.w as i32, x_scale),
			scale_u(self.styles.dimensions.h as i32, y_scale),
		)
	}
	pub fn get_rect(&self) -> Rect {
		self.styles.dimensions
	}
	pub fn move_box(&mut self, x: i32, y: i32) {
		self.styles.dimensions.offset(x, y);
		self.initial_styles.dimensions.offset(x, y);
		if self.text_node.is_some() {
			self.text_node.as_mut().unwrap().move_text(x, y);
		}
	}	
}

impl Renderable for UIBox {
	fn render(&self, 
		canvas: &mut WindowCanvas,
		textures: &HashMap<String, Texture>,
    	fonts: &HashMap<String, Font>,
		x_scale: f64,
		y_scale: f64,
		debug: bool
	) {
		canvas.set_draw_color(self.styles.color);
		canvas.fill_rect(self.get_scaled_rect(x_scale, y_scale)).unwrap();
		if self.text_node.is_some() {
			self.text_node.as_ref().unwrap().render(canvas, textures, fonts, x_scale, y_scale, debug);
		}
	}
}

impl MouseActions for UIBox {
	fn contains_point(&self, x: i32, y: i32) -> bool {
		self.get_rect().contains_point((x, y))
	}
	fn mouse_down(&mut self, x: i32, y: i32) -> Option<EngineFn> {
		if self.contains_point(x, y) {
			self.mouse_details.clicked = true;
		}
		None
	}
	fn mouse_up(&mut self, x: i32, y: i32) -> Option<EngineFn> {
		if self.contains_point(x, y) && self.mouse_details.clicked {
			self.mouse_details.clicked = false;
			return self.on_click.clone();
		}
		None
	}
	fn mouse_move(&mut self, x: i32, y: i32) -> Option<EngineFn> {
		if self.contains_point(x, y) {
			self.styles.color.r = 255 - (255 - self.initial_styles.color.r) / 2 ;
			self.styles.color.g = 255 - (255 - self.initial_styles.color.g) / 2 ;
			self.styles.color.b = 255 - (255 - self.initial_styles.color.b) / 2 ;
		} else {
			self.styles = self.initial_styles.clone()
		}
		None
	}
}

impl HasId for UIBox {
	fn id(&self) -> String {
		self.id.clone()
	}
}

// enum UIElement {
	// Text(TextElement),
	// Box(BoxElement),
// }
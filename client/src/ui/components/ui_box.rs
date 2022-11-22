use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture, BlendMode};
use sdl2::ttf::Font;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::graphics::{scale, scale_u, Renderable};
use crate::ui::{styles::UIStyles};
use crate::input::{MouseActions, MouseDetails};
use crate::entities::{HasId};
use crate::engine::{EngineFn};
use crate::utils::new_id;

use super::text_element::TextElement;

#[derive(Component, Clone)]
pub struct UIBox {
	id: String,
	child_slugs: Vec<String>,
	pub text_node: Option<TextElement>,
	pub styles: UIStyles,
	pub dimensions: Rect,
	pub initial_styles: UIStyles,
	pub mouse_details: MouseDetails,
	pub z_index: i32,
	on_click: Option<EngineFn>,
	game_scale: bool,
	display: bool,
}


impl UIBox {
	pub fn new(
		// id: String,
		child_slugs: Vec<String>,
		text_node: Option<TextElement>,
		dimensions: Rect,
		styles: UIStyles,
		on_click: Option<EngineFn>,
	) -> Self {
		UIBox::new_game_scale(child_slugs, text_node, dimensions, styles, on_click, false)
	}
	pub fn new_game_scale(
		// id: String,
		child_slugs: Vec<String>,
		text_node: Option<TextElement>,
		dimensions: Rect,
		styles: UIStyles,
		on_click: Option<EngineFn>,
		game_scale: bool
	) -> Self {
		Self {
			id: new_id(),
			child_slugs,
			text_node,
			styles: styles.clone(),
			initial_styles: styles,
			mouse_details: MouseDetails::new(),
			z_index: 10000,
			on_click,
			display: true,
			game_scale,
			dimensions
		}
	}
	pub fn get_scaled_rect(&self, pos_x_scale: f64, pos_y_scale: f64, size_x_scale: f64, size_y_scale: f64) -> Rect {
		Rect::from_center((
				scale(self.dimensions.center().x, pos_x_scale), 
				scale(self.dimensions.center().y, pos_y_scale),
			),
			scale_u(self.dimensions.w as i32, size_x_scale),
			scale_u(self.dimensions.h as i32, size_y_scale),
		)
	}
	pub fn get_rect(&self) -> Rect {
		self.dimensions
	}
	pub fn move_box(&mut self, x: i32, y: i32) {
		self.dimensions.offset(x, y);
		if self.text_node.is_some() {
			self.text_node.as_mut().unwrap().move_text(x, y);
		}
	}
	pub fn set_display(&mut self, display: bool) {
		self.display = display;
	}
	pub fn display(&self) -> bool {
		self.display
	}
	pub fn get_child_slugs(&self) -> Vec<String> {
		self.child_slugs.clone()
	}
}

impl Renderable for UIBox {
	fn render(&self, 
		canvas: &mut WindowCanvas,
		textures: &HashMap<String, Texture>,
    	fonts: &HashMap<String, Font>,
		x_scale: f64,
		y_scale: f64,
		map_x_scale: f64,
		map_y_scale: f64,
		debug: bool
	) {
		if !self.display {
			return
		}
		canvas.set_draw_color(self.styles.color);
		canvas.set_blend_mode(BlendMode::Blend);
		if self.game_scale {
			canvas.fill_rect(self.get_scaled_rect(map_x_scale, map_y_scale, map_x_scale, map_y_scale)).unwrap();
		} else {
			canvas.fill_rect(self.get_scaled_rect(x_scale, y_scale, x_scale, y_scale)).unwrap();
		}
		if self.text_node.is_some() {
			self.text_node.as_ref().unwrap().render(canvas, textures, fonts, x_scale, y_scale, map_x_scale, map_y_scale, debug);
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
		// if self.contains_point(x, y) {
		// 	self.styles.color.r = 255 - (255 - self.initial_styles.color.r) / 2 ;
		// 	self.styles.color.g = 255 - (255 - self.initial_styles.color.g) / 2 ;
		// 	self.styles.color.b = 255 - (255 - self.initial_styles.color.b) / 2 ;
		// } else {
		// 	self.styles = self.initial_styles.clone()
		// }
		None
	}
}

impl HasId for UIBox {
	fn id(&self) -> String {
		self.id.clone()
	}
}
use std::collections::HashMap;

use sdl2::{rect::Rect, render::{BlendMode, TextureQuery, WindowCanvas, Texture}, ttf::Font};

use crate::{engine::EngineFn, utils::new_id, graphics::{scale, scale_u}};

use super::{UIBox, UIText, UIImage};

pub enum EntityType {
	Box(UIBox),
	Image(UIImage),
	Text(UIText),
}

impl EntityType {
	pub fn mut_unwrap_box(&mut self) -> &mut UIBox {
		match self {
			EntityType::Box(box_) => box_,
			_ => panic!("expected box"),
		}
	}
	pub fn unwrap_box(&self) -> &UIBox {
		match self {
			EntityType::Box(box_) => box_,
			_ => panic!("expected box"),
		}
	}
}

pub enum EntityPosition {
	Relative,
	Absolute,
}

pub struct Entity {
	id: String,
	x: i32,
	y: i32,
	w: u32,
	h: u32,
	rotation: f64,
	children_ids: Vec<String>, 
	parent_id: Option<String>,
	position: EntityPosition,
	display: bool,
	t: EntityType,
	pub z_index: u32
}

impl Entity {
	pub fn new(d: Rect, t: EntityType) -> Self {
		Self {
			id: new_id(),
			x: d.x,
			y: d.y,
			w: d.w as u32,
			h: d.h as u32,
			rotation: 0.0,
			children_ids: vec![],
			parent_id: None,
			position: EntityPosition::Absolute,
			display: true,
			t,
			z_index: 0
		}
	}

	pub fn id(&self) -> String {
		self.id.clone()
	}
	pub fn contains_point(&self, x: i32, y: i32) -> bool {
		false
	}
	pub fn mouse_move(&mut self, x: i32, y: i32) -> Option<EngineFn> {
		None
	}
	pub fn mouse_down(&mut self, x: i32, y: i32) -> Option<EngineFn> {
		None
	}
	pub fn mouse_up(&mut self, x: i32, y: i32) -> Option<EngineFn> {
		None
	}
	pub fn set_w(&mut self, w: u32) {
		self.w = 0;
	}
	pub fn set_h(&mut self, h: u32) {
		self.h = 0;
	}
	pub fn set_x(&mut self, x: i32) {
		self.x = 0;
	}
	pub fn set_y(&mut self, y: i32) {
		self.y = 0;
	}
	pub fn set_rotation(&mut self) {

	}
	pub fn set_pos(&mut self, x: i32, y: i32) {
		self.x = 0;
		self.y = 0;
	}
	pub fn move_pos(&mut self, x: i32, y: i32) {
		self.set_pos(self.x + x, self.y + y);
	}
	pub fn set_display(&mut self, display: bool) {
		self.display = display;
	}
	pub fn get_child_slugs(&self) -> Vec<String> {
		self.children_ids.clone()
	}
	pub fn get_scaled_rect(&self, pos_x_scale: f64, pos_y_scale: f64, size_x_scale: f64, size_y_scale: f64) -> Rect {
		Rect::from_center((
				scale(self.x, pos_x_scale), 
				scale(self.y, pos_y_scale),
			),
			scale_u(self.w as i32, size_x_scale),
			scale_u(self.h as i32, size_y_scale),
		)
	}
	pub fn get_rect(&self) -> Rect {
		Rect::from_center(
			(self.x,
			self.y),
			self.w,
			self.h
		)
	}
	pub fn append_children(&mut self, children: Vec<String>) {
		self.children_ids.append(&mut children.clone());
	}
	pub fn append_child(&mut self, child: String) {
		self.children_ids.push(child.clone());
	}
	pub fn render(
		&mut self,
		canvas: &mut WindowCanvas,
		textures: &HashMap<String, Texture>,
    	fonts: &HashMap<String, Font>,
	) {
		match &self.t {
			EntityType::Box(b) => {
				canvas.set_draw_color(b.color);
				canvas.set_blend_mode(BlendMode::Blend);
				//TODO: Rect rotation, border radius
				canvas.fill_rect(self.get_rect()).unwrap();
			}
			EntityType::Text(t) => {
				let texture_creator = canvas.texture_creator();
				let font = &fonts[&t.font];
				let surface = font.render(&t.text)
					.blended(t.font_color)
					.map_err(|e| e.to_string()).unwrap();
				let texture = texture_creator.create_texture_from_surface(&surface)
					.map_err(|e| e.to_string()).unwrap();
				let TextureQuery { width, height, .. } = texture.query();
				// let rect_x_scale = if self.game_scale { map_x_scale } else { x_scale };
				// let rect_y_scale = if self.game_scale { map_y_scale } else { y_scale };
				// let x = scale(self.dimensions.center().x, rect_x_scale);
				// let y = scale(self.dimensions.center().y, rect_y_scale);
				// let scaler = self.dimensions.h as f64 / height as f64;
				// add padding and proper scaling?
				// let rect_width_scale = if self.game_scale { scaler * map_x_scale } else { scaler * x_scale };
				// let rect_height_scale = if self.game_scale { scaler * map_y_scale } else { scaler * y_scale };
				let dst = Rect::from_center(
					(self.x, self.y), 
					self.w,
					self.h
					// scale_u(width as i32, rect_width_scale),
					// scale_u(height as i32, rect_height_scale),
				);
				canvas.copy_ex(&texture, None, dst, self.rotation, None, false, false).unwrap();
			}
			EntityType::Image(i) => {

				let dst_rect = Rect::from_center(
					(self.x,self.y),
					self.w,
					self.h
				);
				let src_rect = i.crop;

				canvas.copy_ex(&textures[&i.texture_key], src_rect, dst_rect, self.rotation, None, false, false).unwrap();
				// if debug {
				// 	canvas.set_draw_color(Color::RGB(0, 0, 255));
				// 	canvas.draw_rect(graphic.hitbox_dst).unwrap();
				// 	canvas.copy(&textures["circle"], Rect::new(0, 0, 32, 32), graphic.radius_dst).unwrap();
				// }
			}
		}
	}
}
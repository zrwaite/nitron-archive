use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::ttf::Font;

pub fn scale(n: i32, scale: f64) -> i32 {
	(n as f64* scale) as i32
}

pub fn scale_u(n: i32, scale: f64) -> u32 {
	(n as f64 * scale) as u32
}

pub trait Renderable {
	fn render(&self, 
		canvas: &mut WindowCanvas,
		textures: &HashMap<String, Texture>,
    	fonts: &HashMap<String, Font>,
		x_scale: f64,
		y_scale: f64,
		debug: bool
	);
}

pub struct Graphic {
	pub texture_key: String,
	pub src: Rect, 
	pub dst: Rect,
	pub hitbox_dst: Rect,
	pub radius_dst: Rect,
	pub z_index: i32,
}

pub trait HasZIndex {
	fn z_index(&self) -> i32;
}
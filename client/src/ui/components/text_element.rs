use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture, TextureQuery};
use sdl2::ttf::Font;

use crate::ui::UIStyles; 
use crate::assets::FONTS;
use crate::graphics::{Renderable, scale, scale_u};

#[derive(Clone)]
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
				dimensions: Rect::from_center((x, y), 0, font_size as u32),
				padding: 0,
				border_color: Color::RGBA(0, 0, 0, 0),
				color: Color::RGBA(0, 0, 0, 0),
			},
		}
	}
}
impl Renderable for TextElement {
	fn render(&self, 
		canvas: &mut WindowCanvas,
		_textures: &HashMap<String, Texture>,
    	fonts: &HashMap<String, Font>,
		x_scale: f64,
		y_scale: f64,
		_debug: bool
	) {
		let texture_creator = canvas.texture_creator();
		let font = &fonts[&self.font];
		let surface = font.render(&self.text)
			.blended(self.font_color)
			.map_err(|e| e.to_string()).unwrap();
		let texture = texture_creator.create_texture_from_surface(&surface)
			.map_err(|e| e.to_string()).unwrap();
		let TextureQuery { width, height, .. } = texture.query();
		let x = scale(self.styles.dimensions.center().x, x_scale);
		let y = scale(self.styles.dimensions.center().y, y_scale);
		let scaler = self.styles.dimensions.h as f64 / height as f64;
		// add padding and proper scaling?
		let dst = Rect::from_center(
			(x, y), 
			scale_u(width as i32, scaler * x_scale),
			scale_u(height as i32, scaler * y_scale),
		);
		canvas.copy(&texture, None, dst).unwrap();
	}
}
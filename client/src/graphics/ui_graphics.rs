use std::collections::HashMap;

use sdl2::{render::{Texture, TextureQuery, WindowCanvas}, ttf::Font, rect::{Point, Rect}};

use crate::ui::UIElement;

use super::{scale, scale_u};

pub fn render_ui_element(
    canvas: &mut WindowCanvas,
	ui_element: &UIElement,
	textures: &HashMap<String, Texture>,
    fonts: &HashMap<String, Font>,
	x_scale: f64,
	y_scale: f64,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

	match ui_element {
		UIElement::Text(text_element) => {
			let font = &fonts[&text_element.font];
			let surface = font.render(&text_element.text)
				.blended(text_element.font_color)
				.map_err(|e| e.to_string())?;
			let texture = texture_creator.create_texture_from_surface(&surface)
				.map_err(|e| e.to_string())?;
			let TextureQuery { width, height, .. } = texture.query();
			let x = scale(text_element.styles.x, x_scale);
			let y = scale(text_element.styles.y, y_scale);
			let scale = text_element.styles.height as f64 / height as f64;
			// add padding and proper scaling?
			let dst = Rect::from_center(
				Point::new(x, y), 
				scale_u(width as i32, scale * x_scale),
				scale_u(height as i32, scale * y_scale),
			);
			canvas.copy(&texture, None, dst)?;
		}
		UIElement::Image(image_element) => {
			let texture = &textures[&image_element.image];
			let TextureQuery { width, height, .. } = texture.query();
			let x = image_element.styles.x;
			let y = image_element.styles.y;
			let dst = Rect::from_center(Point::new(x as i32, y as i32), width, height);
			canvas.copy(&texture, None, dst)?;
		}
		UIElement::Box(box_element) => {
			canvas.set_draw_color(box_element.color);
			canvas.fill_rect(box_element.get_rect())?;
			for child in &box_element.elements {
				render_ui_element(canvas, child, textures, fonts, x_scale, y_scale)?;
			}
		}
	}
	Ok(())
}
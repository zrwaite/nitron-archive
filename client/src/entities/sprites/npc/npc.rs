use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::ttf::Font;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::assets::TEXTURES;
use crate::utils::{Vector2, Vector3, Vector4};
use crate::graphics::{Renderable, scale, scale_u, Graphic};
use crate::entities::HasId;
use crate::physics::Hitbox;
use crate::entities::MovingSpriteDisplay;
use crate::utils::new_id;

use super::NpcAnimator;

#[derive(Component, Clone)]
pub struct Npc {
	id: String,
	pub display: MovingSpriteDisplay,
	pub pos: Vector2,
	pub vel: Vector2,
	pub animator: NpcAnimator,
	hitbox: Hitbox,
	// pub stats: CharacterStats,
}

impl Npc {
	pub fn new(
		pos: Vector2, 
		size: Vector3,
		texture_key: String,
		frame_region: Rect
	) -> Self {     
		Npc {
			id: new_id(),
			display: MovingSpriteDisplay::new(texture_key, size.to_vector2()),
			pos,
			vel: Vector2::new(0, 0),
			animator: NpcAnimator::new(),
			hitbox: Hitbox {
				w: (size.x as f32 * 0.6) as u32,
				h: size.z as u32,
				y_offset: size.y / 2 - size.z / 2,
				x_offset: 0,
				radius: (size.x + size.y) as u32,
			},
		}
	}
	pub fn hitbox(&self) -> Vector4 {
		self.hitbox.to_v4(self.pos)
	}
	pub fn set_x_by_hitbox(&mut self, x: i32) {
		self.pos.x = x - self.hitbox.x_offset;
	}
	pub fn set_y_by_hitbox(&mut self, y: i32) {
		self.pos.y = y - self.hitbox.y_offset;
	}
	pub fn contains_point(&self, x: i32, y: i32) -> bool {
		let rect = Rect::new(
			self.pos.x as i32,
			self.pos.y as i32,
			self.display.size.x as u32,
			self.display.size.y as u32,
		);
		rect.contains_point((x, y))
	}
}

impl Renderable for Npc {
	fn render(&self, 
		canvas: &mut WindowCanvas,
		textures: &HashMap<String, Texture>,
    	_fonts: &HashMap<String, Font>,
		x_scale: f64,
		y_scale: f64,
		debug: bool
	) {
		let texture_key = self.display.texture_key.clone();
		let hitbox = self.hitbox().get_scaled(x_scale, y_scale).to_rect();
		let screen_rect = Rect::from_center(
			(
				scale(self.pos.x, x_scale),
				scale(self.pos.y, y_scale),
			),
			scale_u(self.display.size.x, x_scale),
			scale_u(self.display.size.y, y_scale),
		);

		let graphic = Graphic {
			texture_key: texture_key.to_string(),
			src: self.animator.current_frame,
			dst: screen_rect,
			hitbox_dst: hitbox,
			radius_dst: Rect::from_center((hitbox.center().x, hitbox.center().y), self.hitbox.radius, self.hitbox.radius),
			z_index: hitbox.y,
		};
		canvas.copy(&textures[&graphic.texture_key], graphic.src, graphic.dst).unwrap();
		if debug {
			canvas.set_draw_color(Color::RGB(0, 0, 255));
			canvas.draw_rect(graphic.hitbox_dst).unwrap();
			canvas.copy(&textures[TEXTURES.circle], Rect::new(0, 0, 32, 32), graphic.radius_dst).unwrap();
		}
	}
}

impl HasId for Npc {
	fn id(&self) -> String {
		self.id.clone()
	}
}
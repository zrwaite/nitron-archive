use sdl2::rect::Rect;

use crate::{utils::{Vector2, Vector4}, graphics::{scale, scale_u}};

#[derive(Debug, Clone)]
pub struct Hitbox {
	pub w: u32,
	pub h: u32,
	pub y_offset: i32,
	pub x_offset: i32,
	pub radius: u32,
}

impl Hitbox {
	pub fn to_v4(&self, pos: Vector2) -> Vector4 {
		Vector4::new(
			pos.x + self.x_offset,
			pos.y + self.y_offset,
			self.w as i32,
			self.h as i32
		)
	} 
}

pub struct InteractionHitbox {
	pub x: i32,
	pub y: i32,
	pub r: u32,
}	

impl InteractionHitbox {
	pub fn from_hitbox(hitbox: &Hitbox, pos: Vector2) -> Self {
		Self {
			x: pos.x + hitbox.x_offset,
			y: pos.y + hitbox.y_offset,
			r: hitbox.radius,
		}
	}
	pub fn to_scaled_rect(&self, x_scale: f64, y_scale: f64) -> Rect {
		Rect::from_center((scale(self.x, x_scale), scale(self.y, y_scale)), scale_u(self.r as i32, x_scale) * 2, scale_u(self.r as i32, y_scale) * 2)
	}
}
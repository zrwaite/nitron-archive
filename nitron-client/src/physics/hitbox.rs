use crate::components::{Vector2, Vector4};

#[derive(Debug)]
pub struct Hitbox {
	pub w: u32,
	pub h: u32,
	pub y_offset: i32,
	pub x_offset: i32,
}

impl Hitbox {
	// pub fn new(w: u32, h: u32, y_offset: i32, x_offset: i32) -> Self {
	// 	Hitbox {
	// 		w,
	// 		h,
	// 		y_offset,
	// 		x_offset,
	// 	}
	// }
	pub fn to_v4(&self, pos: Vector2) -> Vector4 {
		Vector4::new(
			pos.x + self.x_offset,
			pos.y + self.y_offset,
			self.w as i32,
			self.h as i32
		)
	} 
}
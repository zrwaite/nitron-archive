use crate::engine::{EngineFn};

pub trait MouseActions {
	fn contains_point(&self, x: i32, y: i32) -> bool;
	fn mouse_move(&mut self, x: i32, y: i32) -> Option<EngineFn>;
	fn mouse_down(&mut self, x: i32, y: i32) -> Option<EngineFn>;
	fn mouse_up(&mut self, x: i32, y: i32) -> Option<EngineFn>;
}

#[derive(Clone)]
pub struct MouseDetails {
	pub clicked: bool,
}

impl MouseDetails {
	pub fn new() -> Self {
		Self {
			clicked: false,
		}
	}
}
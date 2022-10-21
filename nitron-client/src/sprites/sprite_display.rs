use crate::components::{Vector2, Direction};

#[derive(Debug)]
pub struct SpriteDisplay {
	pub texture_key:String,
	pub size: Vector2,
}

impl SpriteDisplay {
	pub fn new(texture_key: String, size: Vector2) -> Self {
		SpriteDisplay {texture_key,size}
	}
}

pub struct MovingSpriteDisplay {
	pub texture_key: String,
	pub size: Vector2,
	pub direction: Direction,
}

impl MovingSpriteDisplay {
	pub fn new(texture_key: String, size: Vector2) -> Self {
		MovingSpriteDisplay {
			texture_key,
			size,
			direction: Direction::Down,
		}
	}
}
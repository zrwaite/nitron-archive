use sdl2::rect::Rect;

use crate::{components::Vector2, sprites::{StaticObstacle,}, textures::TEXTURES};

pub fn generate_rock(pos: Vector2, size: Vector2) -> StaticObstacle {
	StaticObstacle::new(pos, size, String::from(TEXTURES.obstacles), Rect::new(80, 32, 32, 16))
}
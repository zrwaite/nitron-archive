use sdl2::rect::Rect;

use crate::utils::{Vector2, Vector3};
use crate::entities::sprites::{StaticObstacle};
use crate::assets::TEXTURES;

pub fn generate_rock(pos: Vector2, size: Vector3) -> StaticObstacle {
	StaticObstacle::default_new(pos, size, String::from(TEXTURES.obstacles), Rect::new(80, 32, 32, 16))
}
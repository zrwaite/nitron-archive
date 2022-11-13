use sdl2::rect::Rect;

use crate::utils::{Vector2, Vector3};
use crate::entities::sprites::{StaticObstacle};
use crate::assets::TEXTURES;

pub fn generate_home(pos: Vector2, size: Vector3) -> StaticObstacle {
	StaticObstacle::new(pos, size, String::from(TEXTURES.home), Rect::new(0, 0, 24, 24))
}
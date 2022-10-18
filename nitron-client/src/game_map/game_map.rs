use crate::{sprites::{StaticObstacle, rock::generate_rock}, components::Vector2};

pub struct GameMap {
	pub width: u32,
	pub height: u32,
	pub static_obstacles: Vec<StaticObstacle>,
	// pub tiles: Vec<Tile>,
}

impl GameMap {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			static_obstacles: Vec::from([generate_rock(Vector2::new(200, 200), Vector2::new(40, 20))]),
		}
	}
}
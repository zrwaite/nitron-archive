use specs_derive::Component;
use specs::DenseVecStorage;
use specs::Component;

use super::{Player, StaticObstacle};

#[derive(Component)]
pub enum Entity {
	Player(Player),
	StaticObstacle(StaticObstacle),
}
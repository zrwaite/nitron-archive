use crate::utils::Vector2;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
	pub chunk_slug: String,
	pub block_index: usize,
	pub pos: Vector2,
}
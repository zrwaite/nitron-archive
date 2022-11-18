use crate::utils::Vector2;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
	chunk_slug: String,
	block_index: usize,
	pos: Vector2,
}
use crate::{game::BlockMap, entity_lib::Entity};

use super::nitron_city;

// pub struct ChunkMap {
// 	pub slug: String,
// 	pub block_slugs: [BlockMap; 1],
// }

pub fn load_chunk(chunk_slug: String) -> Option<(Vec<BlockMap>, Vec<Entity>)> {
	match chunk_slug.as_str() {
		"nitron_city" => Some(nitron_city()),
		_ => None
	}
}
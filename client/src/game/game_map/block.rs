/*
h - f
| 
a - b
|   |
c - d
    |
	j
*/

pub struct BlockMap {
	pub slug: String,
	// pub connections: [String; 2],
	pub width: u32,
	pub height: u32,
	pub static_obstacle_ids: Vec<String>,
}
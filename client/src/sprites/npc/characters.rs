use sdl2::rect::Rect;

use crate::{utils::{Vector2, Vector3}, entity_lib::{Entity}};

use super::Npc;

impl Npc {
	pub fn steve(pos: Vector2) -> (Npc, Vec<Entity>) {
		Npc::new(pos, Vector3::new(32, 40, 10), String::from("NPC"), Rect::new(0, 0, 32, 32))
	}
}
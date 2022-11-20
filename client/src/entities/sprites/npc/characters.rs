use sdl2::rect::Rect;

use crate::{utils::{Vector2, Vector3}, assets::TEXTURES, ui::UIBox};

use super::Npc;

pub fn generate_steve(pos: Vector2) -> (Npc, Vec<UIBox>) {
	Npc::new(pos, Vector3::new(32, 40, 10), String::from(TEXTURES.npc), Rect::new(0, 0, 32, 32))
}
use std::collections::HashMap;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::entities::hash_vec::HasId;

use crate::{ui::{UIBox, create_text_button}, utils::Vector2, engine::EngineFn};

#[derive(Clone)]
pub struct PlayerInteraction {
	on: bool,
	popup_id: String
}

impl PlayerInteraction {
	pub fn new(pos: Vector2) -> (Self, Vec<UIBox>) {
		let popup = create_interaction_popup(pos);
		(
			PlayerInteraction {
				popup_id: popup.id(),
				on: false
			},
			vec![popup]
		)
	}
	pub fn on(&mut self, ui_boxes: &mut HashMap<String, &mut UIBox>) {
		self.on = true;
		let popup = ui_boxes.get_mut(&self.popup_id).unwrap();
		popup.set_display(true)
	}
	pub fn off(&mut self, ui_boxes: &mut HashMap<String, &mut UIBox>) {
		self.on = false;
		let popup = ui_boxes.get_mut(&self.popup_id).unwrap();
		popup.set_display(false)
	}
}

fn create_interaction_popup(pos: Vector2) -> UIBox {
	//todo Fix uibox scaling issue
	let mut popup = create_text_button(
		Rect::new(pos.x, pos.y, 50, 50),
		Color::RGB(255, 255, 255),
		"!".to_string(),
		Some(EngineFn::new(|_engine| {
			// println!("Hello world")
		}))
	);
	popup.z_index = 10000;
	popup.set_display(false);
	popup
}
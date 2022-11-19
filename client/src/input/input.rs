use sdl2::{EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::engine::{EngineFn};
use crate::graphics::scale;
use crate::entities::HashVec;

use super::{KeyTracker,MouseActions};

pub fn handle_events (
	event_pump: &mut EventPump, 
	presses: &mut KeyTracker,
	game_entities: &mut HashVec,
	x_scale: f64,
	y_scale: f64
) -> Vec<EngineFn> {
	//TODO handle multiple events
	let mut engine_fns = Vec::new();
	for event in event_pump.poll_iter() {
		match event {
			Event::Quit {..} |
			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
				return vec![EngineFn::quit()];
			},
			Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
				presses.left = true;
			},
			Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
				presses.right = true;
			},
			Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
				presses.up = true;
			},
			Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
				presses.down = true;
			},
			Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
				presses.left = false;
			}   
			Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
				presses.right = false;
			}   
			Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
				presses.up = false;
			}   
			Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
				presses.down = false;
			},
			Event::MouseMotion { 
				// timestamp, window_id, which, mousestate, xrel, yrel 
				x, y, .. } => {
				for game_entity in game_entities.iter_mut() {
					let engine_event = game_entity.mouse_move(scale(x, 1.0/x_scale),scale(y, 1.0/y_scale));
					if engine_event.is_some() && game_entity.enabled() {
						engine_fns.push(engine_event.unwrap());
					}
				}
			},
			Event::MouseButtonDown { 
				// timestamp, window_id, which, mouse_btn, clicks, 
				x, y, .. } => {
				for game_entity in game_entities.iter_mut() {
					let engine_event = game_entity.mouse_down(scale(x, 1.0/x_scale),scale(y, 1.0/y_scale));
					if engine_event.is_some() && game_entity.enabled() {
						engine_fns.push(engine_event.unwrap());
					}
				}
			},
			Event::MouseButtonUp {
				// timestamp, window_id, which, mouse_btn, clicks, 
				x, y, .. } => {
				for game_entity in game_entities.iter_mut() {
					let engine_event = game_entity.mouse_up(scale(x, 1.0/x_scale),scale(y, 1.0/y_scale));
					if engine_event.is_some() && game_entity.enabled() {
						engine_fns.push(engine_event.unwrap());
					}
				}
			},
			_ => {}
		}
	}
	engine_fns
}
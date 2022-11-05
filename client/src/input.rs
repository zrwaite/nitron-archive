use sdl2::{EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::components::KeyTracker;
use crate::graphics::scale;
use crate::processor::EngineEvent;
use crate::ui::UIElement;

pub fn handle_events (
	event_pump: &mut EventPump, 
	presses: &mut KeyTracker,
	ui_elements: &mut Vec<UIElement>,
	x_scale: f64,
	y_scale: f64
) -> EngineEvent {
	for event in event_pump.poll_iter() {
		match event {
			Event::Quit {..} |
			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
				return EngineEvent::Quit;
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
			Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
				println!("{}, {}", x, y);
				println!("{}, {}", scale(x,x_scale), scale(y,y_scale));
				for ui_element in ui_elements.iter_mut() {
					match ui_element {
						UIElement::Box(box_element) => {
							if box_element.contains_point(scale(x, 1.0/x_scale),scale(y, 1.0/y_scale)) {
								box_element.mouse_details.hovering= true;
								println!("hovering");
							} else {
								box_element.mouse_details.hovering= false;
								// println!("not hovering");
							}
						},
						_ => {}
					}
				}
			},
			Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
				for ui_element in ui_elements.iter_mut() {
					match ui_element {
						UIElement::Box(box_element) => {
							if box_element.contains_point(scale(x, 1.0/x_scale),scale(y, 1.0/y_scale)) {
								box_element.mouse_details.clicked = true;
							}
						},
						_ => {}
					}
				}
			},
			Event::MouseButtonUp {
				// timestamp, window_id, which, mouse_btn, clicks, 
				x, y, .. } => {
				for ui_element in ui_elements.iter_mut() {
					match ui_element {
						UIElement::Box(box_element) => {
							if box_element.contains_point(scale(x, 1.0/x_scale),scale(y, 1.0/y_scale)) && box_element.mouse_details.clicked {
								box_element.mouse_details.clicked = false;
								return EngineEvent::Play;
							}
						},
						_ => {}
					}
				}
			},
			_ => {}
		}
	}
	EngineEvent::None
}
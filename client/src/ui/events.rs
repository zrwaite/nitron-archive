use crate::processor::EngineEvent;

use sdl2::pixels::Color;

#[derive(Clone)]
pub enum UIEventFunction {
	StartButtonAnimation(i32),
	Engine(EngineEvent),
	None
}

pub fn process_ui_events(fns: &Vec<UIEventFunction>) -> Result<EngineEvent, String> {
	for fn_ in fns {
		match fn_ {
			UIEventFunction::StartButtonAnimation(element_id) => {

				// box_element.styles.height += 5;
				// box_element.styles.width += 5;
				// box_element.styles.border_color = Color::RGB(255, 255, 255);
			}
			UIEventFunction::None => {}
			UIEventFunction::Engine(event) => return Ok(event.clone())
		}
	}
	Ok(EngineEvent::None)
}
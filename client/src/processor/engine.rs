use std::{collections::HashMap, time::Duration};

use specs::WorldExt;
use sdl2::{EventPump, render::{WindowCanvas, Texture}, ttf::Font};

use crate::{input, components::KeyTracker, graphics};

use super::{Game, StartScreen};

pub enum Engine {
	Start(StartScreen),
	Running(Game),
	Paused,
	Quit,
}

// pub enum EngineState {
// 	Start,
// 	Running,
// 	Paused,
// 	Stopped,
// }

pub enum EngineEvent {
	Quit,
	Play
}


pub fn run_engine(
	engine: &mut Engine, 
	mut event_pump: &mut EventPump, 
	mut presses: KeyTracker,
	canvas: &mut WindowCanvas,
	textures: &HashMap<String, Texture>,
	fonts: &HashMap<String, Font>,
) -> Result<EngineEvent, String> {
	loop {
		let processor = match engine {
			Engine::Start(start_screen) => &mut start_screen.processor,
			Engine::Running(game) => &mut game.processor,
			Engine::Paused => break,
			Engine::Quit => break,
		};

		// Handle input events
		if input::handle_events(&mut event_pump, &mut presses, &mut processor.ui_elements) {
			return Ok(EngineEvent::Quit);
		}
		*processor.world.write_resource() = presses;
	
		// Update
		processor.process();
	
		// Render
		graphics::renderer::render(
			canvas, 
			&textures, 
			&fonts, 
			&processor.ui_elements, 
			processor.world.system_data(), 
			processor.width,
			processor.height,
			true
		)?;
		
		// Time management
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
	Ok(EngineEvent::Quit)
}
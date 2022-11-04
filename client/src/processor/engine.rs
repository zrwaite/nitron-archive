use std::{collections::HashMap, time::Duration};

use specs::WorldExt;
use sdl2::{EventPump, render::{WindowCanvas, Texture}, ttf::Font};

use crate::{input, components::KeyTracker, graphics};

use super::Processor;

pub enum EngineState {
	Start,
	Running,
	Paused,
	Stopped,
}


pub fn run_engine(
	processor: &mut Processor, 
	mut event_pump: &mut EventPump, 
	mut presses: KeyTracker,
	canvas: &mut WindowCanvas,
	textures: &HashMap<String, Texture>,
	fonts: &HashMap<String, Font>,
) -> Result<(), String> {
	'running: loop {

		// Handle input events
		if input::handle_events(&mut event_pump, &mut presses) {
			break 'running;
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
	
		// render(canvas, fonts, textures)?;
		
		// Time management
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
	Ok(())
}
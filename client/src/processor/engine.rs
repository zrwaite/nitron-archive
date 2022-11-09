use std::{collections::HashMap, time::Duration};

use sdl2::{EventPump, render::{WindowCanvas, Texture}, ttf::Font};
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;
use crate::{input, components::{KeyTracker}, graphics, events::EngineEvent, models::HashVec, controller::{run_controller}, physics::{run_physics}, animation::{run_animator}};

use super::{Game, StartScreen};


#[derive(Component)]
pub enum EngineState {
	Start(StartScreen),
	Playing(Game),
	Quit
}

pub struct Engine {
	pub state: EngineState,
	pub game_entities: HashVec,
	pub width: u32, 
	pub height: u32,
	pub presses: KeyTracker,
}

impl Engine {
	pub fn new(
		width: u32,
		height: u32,
		presses: KeyTracker, 
	) -> Self {
		let (start_screen, entities) = StartScreen::new(width, height);

		let game_entities = HashVec::new(entities);
		
		Self {
			game_entities,
			width,
			height,
			state: EngineState::Start(start_screen),
			presses,
		}
	}
	pub fn run(
		&mut self, 
		mut event_pump: &mut EventPump, 
		mut presses: KeyTracker,
		canvas: &mut WindowCanvas,
		textures: &HashMap<String, Texture>,
		fonts: &HashMap<String, Font>,
	) -> Result<EngineEvent, String> {
		loop {
	
			let (screen_width, screen_height) = canvas.output_size()?;
	
			let x_scale = screen_width as f64 / self.width as f64;
			let y_scale = screen_height as f64 / self.height as f64;

			// Handle input events
			match input::handle_events(
				&mut event_pump, 
				&mut presses, 
				&mut self.game_entities,
				x_scale,
				y_scale
			).unwrap() {
				EngineEvent::Quit => return Ok(EngineEvent::Quit),
				EngineEvent::Play => return Ok(EngineEvent::Play),
				EngineEvent::None => (),
			}
		
			// Update
			run_animator(&mut self.game_entities, &mut self.state);
			run_physics(&mut self.game_entities, &mut self.state);
			run_controller(&mut presses, &mut self.game_entities, &mut self.state);
		
			// Render
			graphics::renderer::render(
				canvas, 
				&textures, 
				&fonts, 
				&self.game_entities, 
				self.width,
				self.height,
				true
			)?;
			
			// Time management
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}
}	


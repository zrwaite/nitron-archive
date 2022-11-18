use std::collections::HashMap;
use std::time::Duration;
use sdl2::EventPump;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::ttf::Font;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::animation::run_animator;
use crate::input::KeyTracker;
use crate::graphics::render;
use crate::input::handle_events;
use crate::entities::HashVec;
use crate::controller::run_controller;
use crate::physics::run_physics;
use crate::Game;

use super::{StartScreen};


#[derive(Component)]
pub enum EngineState {
	Start(StartScreen),
	Playing(Game),
	_Quit
}

#[derive(Clone)]
pub enum EngineEvent {
	Quit,
	Play,
	None
}

#[derive(Clone)]
pub struct EngineFn {
	func: fn (engine: &mut Engine) -> EngineEvent
}
impl EngineFn {
	pub fn new(func: fn (engine: &mut Engine) -> EngineEvent) -> Self {
		Self {
			func
		}
	}
	pub fn run(&self, engine: &mut Engine) -> EngineEvent {
		(self.func)(engine)
	}
	pub fn empty_new(event: EngineEvent) -> Self {
		match event {
			EngineEvent::Quit => Self::new(|_| {
				EngineEvent::Quit
			}),
			EngineEvent::Play => Self::new(|_| {
				EngineEvent::Play
			}),
			EngineEvent::None => Self::new(|_| {
				EngineEvent::None
			})
		}
	}
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
			let engine_fns = handle_events(
				&mut event_pump, 
				&mut presses, 
				&mut self.game_entities,
				x_scale,
				y_scale
			);
			for engine_fn in engine_fns.into_iter() {
				match engine_fn.run(self) {
					EngineEvent::Quit => return Ok(EngineEvent::Quit),
					EngineEvent::Play => return Ok(EngineEvent::Play),
					EngineEvent::None => (),
				}
			}
		
			// Update
			run_animator(&mut self.game_entities, &mut self.state);
			run_physics(&mut self.game_entities, &mut self.state);
			run_controller(&mut presses, &mut self.game_entities, &mut self.state);
		
			// Render
			render(
				canvas, 
				&textures, 
				&fonts, 
				&mut self.game_entities, 
				self.width,
				self.height,
				true
				// false
			)?;
			
			// Time management
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}
}	


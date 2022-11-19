use std::collections::HashMap;
use std::time::Duration;
use sdl2::EventPump;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::ttf::Font;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::animation::run_animator;
use crate::data::read_game_data;
use crate::input::KeyTracker;
use crate::graphics::render;
use crate::input::handle_events;
use crate::entities::{HashVec, GameEntity};
use crate::controller::run_controller;
use crate::physics::run_physics;
use crate::{Game, GAME_WIDTH, GAME_HEIGHT};

use super::{StartScreen};


#[derive(Component)]
pub enum EngineState {
	Start(StartScreen),
	Playing(Game),
}

impl EngineState {
	pub fn mut_unwrap_game(&mut self) -> &mut Game {
		match self {
			EngineState::Playing(game) => game,
			_ => panic!("expected game"),
		}
	}
}

#[derive(Clone)]
pub struct EngineFn {
	func: fn (engine: &mut Engine)
}
impl EngineFn {
	pub fn new(func: fn (engine: &mut Engine)) -> Self {
		Self {
			func
		}
	}
	pub fn run(&self, engine: &mut Engine) {
		(self.func)(engine)
	}
	pub fn quit() -> Self {
		Self::new(|engine| {
			engine.quit();
		})
	}
}

pub struct Engine {
	pub state: EngineState,
	pub game_entities: HashVec,
	pub presses: KeyTracker,
	quitting: bool,
	paused: bool,
}

impl Engine {
	pub fn quit(&mut self) {
		self.quitting = true;
	}
	pub fn pause(&mut self) {
		self.paused = true;
		let pause_menu_id = self.state.mut_unwrap_game().pause_menu_id.clone();
		let pause_menu = self.game_entities.get(pause_menu_id).unwrap().mut_unwrap_box();
		pause_menu.set_display(true);
		let pause_menu_slugs = pause_menu.get_child_slugs();
		for child_slug in pause_menu_slugs {
			let child = self.game_entities.get(child_slug.to_string()).unwrap();
			child.mut_unwrap_box().set_display(true);
		}
		let game_screen_id = self.state.mut_unwrap_game().game_screen_id.clone();
		let game_screen = self.game_entities.get(game_screen_id).unwrap().mut_unwrap_box();
		game_screen.set_display(false);
		let game_screen_slugs = game_screen.get_child_slugs();
		for child_slug in game_screen_slugs {
			let child = self.game_entities.get(child_slug.to_string()).unwrap();
			child.mut_unwrap_box().set_display(false);
		}
	}
	pub fn unpause(&mut self) {
		self.paused = false;
		let pause_menu_id = self.state.mut_unwrap_game().pause_menu_id.clone();
		let pause_menu = self.game_entities.get(pause_menu_id).unwrap().mut_unwrap_box();
		pause_menu.set_display(false);
		let pause_menu_slugs = pause_menu.get_child_slugs();
		for child_slug in pause_menu_slugs {
			let child = self.game_entities.get(child_slug.to_string()).unwrap();
			child.mut_unwrap_box().set_display(false);
		}
		let game_screen_id = self.state.mut_unwrap_game().game_screen_id.clone();
		let game_screen = self.game_entities.get(game_screen_id).unwrap().mut_unwrap_box();
		game_screen.set_display(true);
		let game_screen_slugs = game_screen.get_child_slugs();
		for child_slug in game_screen_slugs {
			let child = self.game_entities.get(child_slug.to_string()).unwrap();
			child.mut_unwrap_box().set_display(true);
		}
	}
	pub fn new(
		presses: KeyTracker, 
	) -> Self {
		let (start_screen, entities) = StartScreen::new();

		let game_entities = HashVec::new(entities);
		
		Self {
			game_entities,
			state: EngineState::Start(start_screen),
			presses,
			quitting: false,
			paused: false,
		}
	}
	pub fn run(
		&mut self, 
		mut event_pump: &mut EventPump, 
		mut presses: KeyTracker,
		canvas: &mut WindowCanvas,
		textures: &HashMap<String, Texture>,
		fonts: &HashMap<String, Font>,
	) -> bool {
		loop {
			if self.quitting {
				return true;
			}
	
			let (screen_width, screen_height) = canvas.output_size().unwrap();
	
			let x_scale = screen_width as f64 / GAME_WIDTH as f64;
			let y_scale = screen_height as f64 / GAME_HEIGHT as f64;

			// Handle input events
			let engine_fns = handle_events(
				&mut event_pump, 
				&mut presses, 
				&mut self.game_entities,
				x_scale,
				y_scale
			);
			for engine_fn in engine_fns.into_iter() {
				engine_fn.run(self)
			}
		
			// Update
			if !self.paused {
				run_animator(&mut self.game_entities, &mut self.state);
				run_physics(&mut self.game_entities, &mut self.state);
				run_controller(&mut presses, &mut self.game_entities, &mut self.state);
			}

			let (width, height) = self.size();
			// Render
			render(
				canvas, 
				&textures, 
				&fonts, 
				&mut self.game_entities, 
				width,
				height,
				true
				// false
			).unwrap();
			
			// Time management
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}
	fn initialize_game(&mut self, game: Game, game_entities: Vec<GameEntity>) {
		self.game_entities.clear();
		self.game_entities = HashVec::new(game_entities);
		self.state = EngineState::Playing(game);
	}
	pub fn new_game(&mut self) {
		let (game, game_entities) = Game::new();
		self.initialize_game(game, game_entities);
	}
	pub fn continue_game(&mut self) {
		let game_data = read_game_data().unwrap();
		let (game, game_entities) = Game::from(game_data);
		self.initialize_game(game, game_entities);
	}
	pub fn size(&self) -> (u32, u32) {
		match self.state {
			EngineState::Start(_) => (GAME_WIDTH, GAME_HEIGHT),
			EngineState::Playing(ref game) => (game.block().width, game.block().height)
		}
	}
}	


use std::time::Duration;
use sdl2::EventPump;
use specs_derive::Component;
use specs::Component;
use specs::DenseVecStorage;

use crate::animation::run_animator;
use crate::data::read_game_data;
use crate::entity_lib::EntityScreen;
use crate::entity_lib::{EntityStore, Entity};
use crate::input::KeyTracker;
use crate::input::handle_events;
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
	pub entity_store: EntityStore,
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
		let pause_menu = self.entity_store.get(pause_menu_id).unwrap();
		pause_menu.set_display(true);
		let pause_menu_slugs = pause_menu.get_child_slugs();
		for child_slug in pause_menu_slugs {
			let child = self.entity_store.get(child_slug.to_string()).unwrap();
			child.set_display(true);
		}
		let game_screen_id = self.state.mut_unwrap_game().game_screen_id.clone();
		let game_screen = self.entity_store.get(game_screen_id).unwrap();
		game_screen.set_display(false);
		let game_screen_slugs = game_screen.get_child_slugs();
		for child_slug in game_screen_slugs {
			let child = self.entity_store.get(child_slug.to_string()).unwrap();
			child.set_display(false);
		}
	}
	pub fn unpause(&mut self) {
		self.paused = false;
		let pause_menu_id = self.state.mut_unwrap_game().pause_menu_id.clone();
		let pause_menu = self.entity_store.get(pause_menu_id).unwrap();
		pause_menu.set_display(false);
		let pause_menu_slugs = pause_menu.get_child_slugs();
		for child_slug in pause_menu_slugs {
			let child = self.entity_store.get(child_slug.to_string()).unwrap();
			child.set_display(false);
		}
		let game_screen_id = self.state.mut_unwrap_game().game_screen_id.clone();
		let game_screen = self.entity_store.get(game_screen_id).unwrap();
		game_screen.set_display(true);
		let game_screen_slugs = game_screen.get_child_slugs();
		for child_slug in game_screen_slugs {
			let child = self.entity_store.get(child_slug.to_string()).unwrap();
			child.set_display(true);
		}
	}
	pub fn new() -> Self {

		let (start_screen, entities) = StartScreen::new();

		let entity_store = EntityStore::from(entities);
		
		Self {
			entity_store,
			state: EngineState::Start(start_screen),
			presses:  KeyTracker::new(),
			quitting: false,
			paused: false,
		}
	}
	pub fn run(
		&mut self, 
		mut event_pump: &mut EventPump, 
		mut entity_screen: &mut EntityScreen,
	) -> bool {
		loop {
			if self.quitting {
				return true;
			}

			// Handle input events
			let engine_fns = handle_events(
				&mut event_pump, 
				&mut self.presses, 
				&mut self.entity_store,
			);
			for engine_fn in engine_fns.into_iter() {
				engine_fn.run(self)
			}
		
			// Update
			if !self.paused {
				run_animator(&mut self.entity_store, &mut self.state);
				run_physics(&mut self.entity_store, &mut self.state);
				match &mut self.state {
					EngineState::Playing(game) => {
						(&mut game.player).run_controller(&mut self.presses);
					}
					_ => {}
				}
			}
			// Time management
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}
	fn initialize_game(&mut self, game: Game, entity_store: Vec<Entity>) {
		self.entity_store.clear();
		self.entity_store = EntityStore::from(entity_store);
		self.state = EngineState::Playing(game);
	}
	pub fn new_game(&mut self) {
		let (game, entity_store) = Game::new();
		self.initialize_game(game, entity_store);
	}
	pub fn continue_game(&mut self) {
		let game_data = read_game_data().unwrap();
		let (game, entity_store) = Game::from(game_data);
		self.initialize_game(game, entity_store);
	}
	pub fn size(&self) -> (u32, u32) {
		match self.state {
			EngineState::Start(_) => (GAME_WIDTH, GAME_HEIGHT),
			EngineState::Playing(ref game) => (game.block().width, game.block().height)
		}
	}
}	


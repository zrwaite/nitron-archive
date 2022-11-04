use sdl2::pixels::Color;
use specs::{World, DispatcherBuilder};
use specs::{WorldExt,Builder,Component};
use specs_derive::Component;
use crate::entities::Entity;
use crate::ui::{TextElement, UIElement};
use crate::{entities::player::Player, game_map::GameMap};
use specs::DenseVecStorage;
use specs::SystemData;

use crate::components::{KeyboardControlled, KeyTracker};
use crate::{controller, physics, animation, graphics};

use super::{Processor, ProcessorTrait, ProcessorData};

#[derive(Component)]
pub struct GameData {
	pub player: Player,
	pub entities: Vec<Entity>,
	pub map: GameMap,
}

pub struct Game {
	pub processor: Processor,
	// pub data: GameData,
}

impl Game {
	pub fn new(
		game_width: u32, 
		game_height: u32,
		player: Player,
		presses: KeyTracker,
	) -> Self {
		let data = GameData {
			map: GameMap::new(game_width, game_height),
			entities: vec![],
			player,
		};
		let processor = Game::new_processor(presses, ProcessorData::Game(data), game_width, game_height);
		Self {
			processor,
		}
	}
	
}


impl ProcessorTrait for Game {
	fn new_processor(presses: KeyTracker, data: ProcessorData, width: u32, height: u32) -> Processor {
		let mut dispatcher =  DispatcherBuilder::new()
			.with(controller::Controller, "Controller", &[])
			.with(physics::Physics{}, "Physics", &["Controller"])
			.with(animation::Animator, "Animator", &["Controller"])
			.build();
		let mut world = World::new();
		dispatcher.setup(&mut world);
		graphics::renderer::SystemData::setup(&mut world);
		world.insert(presses);
		world.create_entity()
			.with(KeyboardControlled)
			.with(data)
			.build();
		
		Processor {
			dispatcher,
			world,
			ui_elements: vec![
				UIElement::Text(TextElement::new_normal("Hello World".to_string(), 30, Color::RGB(255, 255, 255), width as i32/2, height as i32/2))
			],
			width,
			height,
		}
	}

	// fn render(&mut self, canvas: &mut WindowCanvas, fonts: &HashMap<String, Font>, textures: &HashMap<String, Texture>) -> Result<(), String> {
	// 	graphics::renderer::render(
	// 		canvas, 
	// 		&textures, 
	// 		&fonts, 
	// 		&self.processor.ui_elements, 
	// 		self.processor.world.system_data(), 
	// 		self.processor.width,
	// 		self.processor.height,
	// 		true
	// 	)
	}

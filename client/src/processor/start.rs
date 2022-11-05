use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use specs::{World, DispatcherBuilder};
use specs::{WorldExt,Builder};
use crate::ui::components::create_text_button;
use specs::SystemData;

use crate::components::{KeyboardControlled, KeyTracker};
use crate::{controller, animation, graphics};

use super::{Processor, ProcessorTrait, ProcessorData, EngineEvent};

pub struct StartScreen {
	pub processor: Processor,
}

impl StartScreen {
	pub fn new(
		screen_width: u32, 
		screen_height: u32,
		presses: KeyTracker,
	) -> Self {
		let processor = StartScreen::new_processor(presses, ProcessorData::None, screen_width, screen_height);
		Self {
			processor,
		}
	}
	
}


impl ProcessorTrait for StartScreen {
	fn new_processor(presses: KeyTracker, data: ProcessorData, width: u32, height: u32) -> Processor {
		let mut dispatcher =  DispatcherBuilder::new()
			.with(controller::Controller, "Controller", &[])
			// .with(physics::Physics{}, "Physics", &["Controller"])
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
				create_text_button(
					Rect::from_center(Point::new(width as i32/2, height as i32/2), 80, 40),
					Color::RGB(0, 200, 150),
					EngineEvent::Play,
					"Start".to_string(),
				),
			],
			width,
			height,
		}
	}
}

use sdl2::render::{Texture, WindowCanvas};
use specs::{Dispatcher, World};
use specs::{WorldExt,Component};
use specs_derive::Component;
use std::collections::HashMap;
use specs::DenseVecStorage;

use crate::components::{KeyTracker};

use super::GameData;

pub struct Processor {
	pub dispatcher: Dispatcher<'static, 'static>,
	pub world: World,
}

impl Processor {
	pub fn process(&mut self) {
		self.dispatcher.dispatch(&self.world);
		self.world.maintain();
	}
}

#[derive(Component)]
pub enum ProcessorData {
	Game(GameData),
}

pub trait ProcessorTrait {
	fn new_processor(presses: KeyTracker, data: ProcessorData) -> Processor;
	fn render(&mut self, canvas: &mut WindowCanvas, textures: &HashMap<String, Texture>) -> Result<(), String>;
}
use specs::{Dispatcher, World};
use specs::{WorldExt,Component};
use specs_derive::Component;
use specs::DenseVecStorage;

use crate::components::{KeyTracker};
use crate::ui::UIElement;

use super::GameData;

pub struct Processor {
	pub dispatcher: Dispatcher<'static, 'static>,
	pub world: World,
	pub ui_elements: Vec<UIElement>,
	pub width: u32,
	pub height: u32,
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
	fn new_processor(presses: KeyTracker, data: ProcessorData, width: u32, height: u32) -> Processor;
}
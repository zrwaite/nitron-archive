use std::collections::HashMap;
use std::path::PathBuf;

use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::ttf::{self, Font, Sdl2TtfContext};
use sdl2::image::{LoadTexture, self, InitFlag};
use sdl2::video::WindowContext;
use sdl2::Sdl;

use super::EntityStore;

pub struct EntityScreen<'a> {
	store: EntityStore,
	canvas: WindowCanvas,
	sdl_context: Sdl,
	ttf_context: Sdl2TtfContext,
	textures: HashMap<String, Texture<'a>>,
	texture_creator: TextureCreator<WindowContext>,
	fonts: HashMap<String, Font<'a, 'a>>,
	h: u32,
	w: u32,
}

struct GraphicIndex {
    pub id: String,
    pub z_index: u32,
}

impl EntityScreen<'_> {
	pub fn new(title: &str, h: u32, w: u32) -> Self {
		let sdl_context = sdl2::init().unwrap();
		let video_subsystem = sdl_context.video().unwrap();
		let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
		let window = video_subsystem.window(title, w, h)
		.position_centered()
		.resizable()
		.build()
		.expect("could not initialize video subsystem");
		// TODO set mimimum resize

		let mut canvas = window.into_canvas().build().expect("could not make a canvas");
		let texture_creator = canvas.texture_creator();
		let ttf_context = ttf::init().map_err(|e| e.to_string()).unwrap();
		Self {
			sdl_context,
			store: EntityStore::empty(),
			canvas,
			ttf_context,
			textures: HashMap::new(),
			texture_creator,
			fonts: HashMap::new(),
			h,
			w
		}
	}
	pub fn get_event_pump(&self) -> sdl2::EventPump {
		self.sdl_context.event_pump().unwrap()
	}
	pub fn load_textures(&mut self, map: Vec<(String, PathBuf)>) {
		for texture in map.iter() {
			let loaded_texture = self.texture_creator.load_texture(texture.1.clone()).unwrap();
			self.textures.insert(
				texture.0.clone(), 
				loaded_texture
			);
		}
	}
	pub fn load_fonts(&mut self, map: Vec<(String, PathBuf)>) {
		for font in map.iter() {
			self.fonts.insert(font.0.clone(), self.ttf_context.load_font(font.1, 128).unwrap());
		}
	}
	pub fn render(&mut self) {
		self.canvas.clear();
		let (screen_width, screen_height) = self.canvas.output_size().unwrap();
		

		// TODO: Hargun, make this more efficien:
		let mut element_graphics: Vec<GraphicIndex> = Vec::new();
		for element in self.store.iter() {
			element_graphics.push(GraphicIndex {
				id: element.id().clone(),
				z_index: element.z_index,
			})
		}
		// TODO: Specifically, prevent this sort from happening every frame
		element_graphics.sort_by(|a, b| a.z_index.cmp(&b.z_index));
	
		for element_graphic in element_graphics {
			let element = self.store.get(element_graphic.id).unwrap();
			element.render(
				&mut self.canvas,
				&self.textures,
				&self.fonts
			);
		}
	
		self.canvas.present();
	}
}
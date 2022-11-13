use std::collections::HashMap;
use sdl2::{render::{TextureCreator, Texture}, video::WindowContext};
use sdl2::image::LoadTexture;

pub struct StringTextures {
	pub player: String,
	pub obstacles: String,
	pub home: String,
	// pub game_map: GameMapTextures,
}

pub struct Textures {
	pub player: &'static str,
	pub obstacles: &'static str,
	pub home: &'static str,
	// pub game_map: GameMapTextures,
}

impl Textures {
	pub fn to_strings(&self) -> StringTextures {
		StringTextures {
			player: self.player.to_string(),
			obstacles: self.obstacles.to_string(),
			home: self.home.to_string(),
		}
	}
}

pub static TEXTURES: Textures = Textures {
	player: "Player.png",
	obstacles: "darkdimension.png",
	home: "Home.png",
	// game_map: GameMapTextures::new(),
};

pub fn load_textures(binary_filepath: String, texture_creator: &TextureCreator<WindowContext>) -> HashMap<String, Texture> {
	let mut textures = HashMap::new();
	let string_textures = TEXTURES.to_strings();
	textures.insert(string_textures.player.to_string(), texture_creator.load_texture(binary_filepath.clone() + &string_textures.player).unwrap());
	textures.insert(string_textures.obstacles.to_string(), texture_creator.load_texture(binary_filepath.clone() + &string_textures.obstacles).unwrap());
	textures.insert(string_textures.home.to_string(), texture_creator.load_texture(binary_filepath.clone() + &string_textures.home).unwrap());
	textures
}
pub struct Textures {
	pub player: &'static str,
	pub obstacles: &'static str,
	// pub game_map: GameMapTextures,
}

pub static TEXTURES: Textures = Textures {
	player: "assets/bardo.png",
	obstacles: "assets/darkdimension.png",
	// game_map: GameMapTextures::new(),
};

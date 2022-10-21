pub struct Textures {
	pub player: &'static str,
	pub obstacles: &'static str,
	pub debug_box: &'static str,
	// pub game_map: GameMapTextures,
}

pub static TEXTURES: Textures = Textures {
	player: "assets/bardo.png",
	obstacles: "assets/darkdimension.png",
	debug_box: "assets/debug_box.png",
	// game_map: GameMapTextures::new(),
};

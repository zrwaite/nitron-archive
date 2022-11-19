use std::{fs::File, io::BufReader};

use super::GameData;

pub fn write_game_data(game_data: &GameData) {
	let file = File::create("game_data.json").unwrap();
	serde_json::to_writer(file, game_data).unwrap();
}

pub fn game_data_exists() -> bool {
	let file_name = "game_data.json";
	match File::open(file_name){
		Ok(_) => true,
		Err(_) => false
	}
}

pub fn read_game_data() -> Result<GameData, String> {
	let file_name = "game_data.json";
	let file = File::open(file_name).map_err(|e| e.to_string())?;
	let reader = BufReader::new(file);
	let game_data: GameData = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
	Ok(game_data)
}

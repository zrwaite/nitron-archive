mod physics;
mod animation;
mod graphics;
mod input;
mod ui_components;
mod sprites;
mod utils;
mod engine;
mod entity_lib;
mod game;
mod data;

use std::env;
use std::path::PathBuf;
use entity_lib::EntityScreen;
use sdl2::image::{self, InitFlag};

use engine::Engine;
use game::Game;

const GAME_WIDTH: u32 = 800;
const GAME_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut assets_prefix = PathBuf::from("client/assets/");
    let binary_filepath = PathBuf::from(&args[0]);

    #[cfg(target_family = "windows")]
    if binary_filepath.ends_with("target/debug/nitron-client.exe") {
        println!("Running in dev mode");
    }

    #[cfg(target_os = "macos")]
    if binary_filepath.ends_with("target/debug/nitron-client") {
        println!("Running in dev mode");
    } else {
        assets_prefix = binary_filepath.parent().unwrap().join("assets/");
    }

    #[cfg(target_family = "unix")]
    if binary_filepath.ends_with("target/debug/nitron-client") {
        println!("Running in dev mode");
    } else {
        // Fill in if there's any Linux specific stuff
    }

    

    // initialize textures
    // let texture_creator = canvas.texture_creator();


    let mut entity_screen = EntityScreen::new("Nitron", GAME_WIDTH, GAME_HEIGHT);
    entity_screen.load_textures(
        // &texture_creator,
        vec![
        ("player".to_string(), assets_prefix.join("Player.png".to_string())),
        ("obstacles".to_string(), assets_prefix.join("darkdimension.png".to_string())),
        ("home".to_string(), assets_prefix.join("Home.png".to_string())),
        ("npc".to_string(), assets_prefix.join("Enemy.png".to_string())),
        ("circle".to_string(), assets_prefix.join("Circle.png".to_string())),
    ]);
    entity_screen.load_fonts(vec![
        ("electrolize".to_string(), assets_prefix.join("Electrolize/Electrolize-Regular.ttf".to_string())),
    ]);

    // Initialize resource
    let mut engine = Engine::new();
    let mut event_pump = entity_screen.get_event_pump();

    engine.run(
        &mut event_pump, 
        &mut entity_screen
    );
    Ok(())
}
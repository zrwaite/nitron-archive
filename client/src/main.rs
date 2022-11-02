mod physics;
mod animation;
mod controller;
mod graphics;
mod components;
mod input;
mod textures;
mod entities;
mod game_map;
mod processor;

use components::{Vector2, KeyTracker, Vector3};
use crate::processor::ProcessorTrait;
use entities::player::Player;

use sdl2::image::{self, InitFlag};

use specs::{WorldExt};
use textures::{TEXTURES, load_textures};

use std::env;
use std::time::Duration;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let assets_prefix;
    let binary_filepath = args[0].clone();
    if binary_filepath == "target/debug/nitron-client" {
        println!("Running in dev mode");
        assets_prefix = "assets/".to_string();
    } else {
        assets_prefix = String::from(binary_filepath).replace("MacOS/nitron-client", "MacOS/assets/");
    }
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("game tutorial", 800, 600)
    .position_centered()
    .resizable()
    .build()
    .expect("could not initialize video subsystem");
    // TODO set mimimum resize

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    // initialize textures
    let texture_creator = canvas.texture_creator();
    let textures = load_textures(assets_prefix, &texture_creator);

    // Initialize resource
    let mut presses = KeyTracker::new();

    
    

    let player = Player::new(Vector2::new(100, 100), Vector3::new(26, 36, 10), String::from(TEXTURES.player));
    let mut game = processor::game::Game::new(400, 300, player, presses);

    //clear world
    // world.delete_all();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle input events
        if input::handle_events(&mut event_pump, &mut presses) {
            break 'running;
        }
        *game.processor.world.write_resource() = presses;

        // Update
        game.processor.process();

        // Render
        game.render(&mut canvas, &textures)?;
        
        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
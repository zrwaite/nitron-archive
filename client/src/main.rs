mod physics;
mod animation;
mod controller;
mod graphics;
mod components;
mod input;
mod assets;
mod entities;
mod game_map;
mod processor;
mod ui;

use std::env;

use components::{Vector2, KeyTracker, Vector3};
use processor::{run_engine, EngineState, Game};
use entities::player::Player;

use sdl2::image::{self, InitFlag};

use assets::{TEXTURES, load_textures, load_fonts};


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
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("game tutorial", 800, 600)
    .position_centered()
    .resizable()
    .build()
    .expect("could not initialize video subsystem");
    // TODO set mimimum resize

    let mut engine_state = EngineState::Start;

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    // initialize textures
    let texture_creator = canvas.texture_creator();
    let textures = load_textures(assets_prefix.clone(), &texture_creator);
    let fonts = load_fonts(String::from(assets_prefix + "fonts/"), &ttf_context);

// pub static mut engine_state:EngineState = EngineState::Start;

    // Initialize resource
    let presses = KeyTracker::new();


    let mut player: Player;
    let mut game: Option<Game> = None;

    
    // let mut processor = game.processor;

    //clear world
    // world.delete_all();

    let mut event_pump = sdl_context.event_pump()?;

    'engine: loop {
        match engine_state {
            EngineState::Start => {
                // run_engine(
                //     &mut game.processor, 
                //     &mut event_pump, 
                //     game.presses, 
                //     &mut canvas, 
                //     &textures, 
                //     &fonts
                // )?;
                player = Player::new(Vector2::new(100, 100), Vector3::new(26, 36, 10), String::from(TEXTURES.player));
                game = Some(processor::game::Game::new(400, 300, player, presses));
                engine_state = EngineState::Running;
            },
            EngineState::Running => {
                run_engine(
                    &mut game.expect("Game not initialized").processor, 
                    &mut event_pump, 
                    presses,
                    &mut canvas, 
                    &textures, 
                    &fonts, 
                )?;
                return Ok(());
            },
            EngineState::Paused => {
                // run_engine(
                //     &mut game.processor, 
                //     &mut event_pump, 
                //     game.presses, 
                //     &mut canvas, 
                //     &textures, 
                //     &fonts
                // )?;
            },
            EngineState::Stopped => {
                break 'engine;
            }
        } 
    }
    Ok(())
}
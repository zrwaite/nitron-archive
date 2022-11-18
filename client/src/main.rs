mod physics;
mod animation;
mod controller;
mod graphics;
mod input;
mod assets;
mod entities;
mod utils;
mod engine;
mod ui;
mod game;
mod data;

use std::env;
use sdl2::image::{self, InitFlag};

use input::{KeyTracker};
use entities::HashVec;
use engine::{Engine, EngineState, EngineEvent};
use game::{Game};
use assets::{load_textures, load_fonts};

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

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    // initialize textures
    let texture_creator = canvas.texture_creator();
    let textures = load_textures(assets_prefix.clone(), &texture_creator);
    let fonts = load_fonts(String::from(assets_prefix + "fonts/"), &ttf_context);

    // Initialize resource
    let presses = KeyTracker::new();

    let mut engine = Engine::new(
        400,
        300,
        presses,
    );

    let mut event_pump = sdl_context.event_pump()?;

    'engine_loop: loop {
        let event_event = engine.run(
            &mut event_pump, 
            presses, 
            &mut canvas, 
            &textures, 
            &fonts
        )?;

        match event_event {
            EngineEvent::Quit => break 'engine_loop,
            EngineEvent::Play => {
                
                let (game, game_entities) = Game::new();
                engine.game_entities.clear();
                engine.game_entities = HashVec::new(game_entities);
                engine.state = EngineState::Playing(game);
            }
            EngineEvent::None => {
                panic!("ended start screen without follow up event")
            }
        }
    }
    Ok(())
}
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
mod events;

use std::env;

use components::{Vector2, KeyTracker, Vector3};
use processor::{run_engine, Game, StartScreen, Engine, EngineEvent};
use entities::player::Player;

use sdl2::{image::{self, InitFlag}};

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

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    // initialize textures
    let texture_creator = canvas.texture_creator();
    let textures = load_textures(assets_prefix.clone(), &texture_creator);
    let fonts = load_fonts(String::from(assets_prefix + "fonts/"), &ttf_context);

    // Initialize resource
    let presses = KeyTracker::new();

    let mut engine = Engine::Start(
        StartScreen::new(
            400, 300, presses
        )   
    );

    let mut event_pump = sdl_context.event_pump()?;

    'engine: loop {
        match engine {
            Engine::Start(start_screen) => {
                let event_event = run_engine(
                    &mut Engine::Start(start_screen), 
                    &mut event_pump, 
                    presses, 
                    &mut canvas, 
                    &textures, 
                    &fonts
                )?;

                match event_event {
                    EngineEvent::Quit => break 'engine,
                    EngineEvent::Play => {
                        let player = Player::new(Vector2::new(100, 100), Vector3::new(26, 36, 10), String::from(TEXTURES.player));
                        let game = Game::new(800, 600, player, presses);
                        engine = Engine::Running(game);
                    }
                    EngineEvent::None => {
                        panic!("ended start screen without follow up event")
                    }
                }
                
            },
            Engine::Running(game) => {
                run_engine(
                    &mut Engine::Running(game), 
                    &mut event_pump, 
                    presses,
                    &mut canvas, 
                    &textures, 
                    &fonts, 
                )?;
                engine = Engine::Quit;
            },
            Engine::Paused => {
                // run_engine(
                //     &mut game.processor, 
                //     &mut event_pump, 
                //     game.presses, 
                //     &mut canvas, 
                //     &textures, 
                //     &fonts
                // )?;
            },
            Engine::Quit => {
                break 'engine;
            }
        } 
    }
    Ok(())
}
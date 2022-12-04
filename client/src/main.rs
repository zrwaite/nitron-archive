mod physics;
mod animation;
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
use std::path::{Path, PathBuf};
use sdl2::image::{self, InitFlag};

use input::KeyTracker;
use engine::Engine;
use game::Game;
use assets::{load_textures, load_fonts};

const GAME_WIDTH: u32 = 800;
const GAME_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut assets_prefix= PathBuf::from("assets/");
    let binary_filepath = PathBuf::from(&args[0]);

    #[cfg(target_family = "windows")]
    if binary_filepath.ends_with("target/debug/nitron-client.exe") {
        println!("Running in dev mode");
    }

    #[cfg(target_family = "unix")]
    if binary_filepath.ends_with("target/debug/nitron-client") {
        println!("Running in dev mode");
    } else {
        #[cfg(target_os = "macos")]
        assets_prefix = binary_filepath.parent().unwrap().join("assets/");
    }

    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("game tutorial", GAME_WIDTH, GAME_HEIGHT)
    .position_centered()
    .resizable()
    .build()
    .expect("could not initialize video subsystem");
    // TODO set mimimum resize

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    // initialize textures
    let texture_creator = canvas.texture_creator();
    let textures = load_textures(assets_prefix.clone(), &texture_creator);
    let fonts = load_fonts(assets_prefix.join("fonts"), &ttf_context);

    // Initialize resource
    let presses = KeyTracker::new();

    let mut engine = Engine::new(presses);

    let mut event_pump = sdl_context.event_pump()?;

    'engine_loop: loop {
        let quit = engine.run(
            &mut event_pump, 
            presses, 
            &mut canvas, 
            &textures, 
            &fonts
        );

        if quit {
            break 'engine_loop;
        }

        // match event_event {
            // EngineEvent::Quit => break 'engine_loop,
            // EngineEvent::Play => {
                
            // }
            // EngineEvent::None => {
            //     panic!("ended start screen without follow up event")
            // }
        // }
    }
    Ok(())
}
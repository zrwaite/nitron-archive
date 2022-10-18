mod physics;
mod animation;
mod controller;
mod renderer;
mod components;
mod input;
mod textures;
mod game;
mod player;
mod sprites;
mod game_map;

use components::{Vector2, KeyboardControlled, KeyTracker};
use player::Player;

use sdl2::image::{self, LoadTexture, InitFlag};

use specs::{WorldExt, Builder, SystemData};
use specs::prelude::{DispatcherBuilder, World};
use textures::TEXTURES;

use std::collections::HashMap;
use std::time::Duration;

fn main() -> Result<(), String> {
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
    
    let mut dispatcher = DispatcherBuilder::new()
        .with(controller::Controller, "Controller", &[])
        .with(physics::Physics{}, "Physics", &["Controller"])
        .with(animation::Animator, "Animator", &["Controller"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);


    // initialize textures
    let texture_creator = canvas.texture_creator();

    let mut textures = HashMap::new();
    textures.insert(String::from(TEXTURES.player),texture_creator.load_texture(TEXTURES.player)?);
    textures.insert(String::from(TEXTURES.obstacles),texture_creator.load_texture(TEXTURES.obstacles)?);

    // Initialize resource
    let mut presses = KeyTracker::new();

    let player = Player::new(Vector2::new(100, 100), Vector2::new(26, 36), String::from(TEXTURES.player));

    let game = game::Game::new(400, 300, player);

    world.insert(presses);
    world.create_entity()
        .with(KeyboardControlled)
        .with(game)
        .build();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle input events
        if input::handle_events(&mut event_pump, &mut presses) {
            break 'running;
        }

        *world.write_resource() = presses;

        // Update
        dispatcher.dispatch(&mut world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, &textures, world.system_data())?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
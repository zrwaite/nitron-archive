use std::collections::HashMap;

use specs::Join;
use specs::prelude::{ReadStorage};
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::game::Game;

pub type SystemData<'a> = (
    ReadStorage<'a, Game>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &HashMap<String, Texture>,
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let game = &data.0.join().next().unwrap();
    let player = &game.player;

    let (width, height) = canvas.output_size()?;

    let x_scale = width as f64 / game.map.width as f64;
    let y_scale = height as f64 / game.map.height as f64;

    let texture_key = &game.player.display.texture_key;
    let current_frame = player.animator.current_frame.region;

    let screen_position = Point::new((player.pos.x as f64 * x_scale) as i32 , (player.pos.y as f64 * y_scale) as i32);
    let screen_rect = Rect::from_center(
        Point::new(
            screen_position.x(),
            screen_position.y(),
        ),
        (player.display.size.x as f64 * x_scale) as u32,
        (player.display.size.y as f64* y_scale) as u32,
    );
    canvas.copy(&textures[texture_key], current_frame, screen_rect)?;

    // iterate over all game map obstacles
    for obstacle in &game.map.static_obstacles {
        let texture_key = &obstacle.display.texture_key;
        let current_frame = obstacle.frame.region;

        let screen_position = Point::new((obstacle.pos.x as f64 * x_scale) as i32 , (obstacle.pos.y as f64 * y_scale) as i32);
        let screen_rect = Rect::from_center(
            Point::new(
                screen_position.x(),
                screen_position.y(),
            ),
            (obstacle.display.size.x as f64 * x_scale) as u32,
            (obstacle.display.size.y as f64* y_scale) as u32,
        );
        canvas.copy(&textures[texture_key], current_frame, screen_rect)?;
    }

    canvas.present();

    Ok(())
}
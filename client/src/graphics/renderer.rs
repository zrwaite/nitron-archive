use std::collections::HashMap;

use specs::Join;
use specs::prelude::{ReadStorage};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;

use crate::processor::{ProcessorData};
use crate::textures::TEXTURES;

use super::get_graphics;

pub type SystemData<'a> = (
    ReadStorage<'a, ProcessorData>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &HashMap<String, Texture>,
    data: SystemData,
    debug: bool,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let (width, height) = canvas.output_size()?;

    let processor_data = &data.0.join().next().unwrap();
    match processor_data {
        ProcessorData::Game(game) => {
            let x_scale = width as f64 / game.map.width as f64;
            let y_scale = height as f64 / game.map.height as f64;
        
            let graphics = get_graphics(game, x_scale, y_scale);
        
            for graphic in graphics {
                canvas.copy(&textures[&graphic.texture_key], graphic.src, graphic.dst)?;
                if debug {
                    canvas.copy(&textures[TEXTURES.debug_box], Rect::new(0,  0, 24, 24), graphic.hitbox_dst)?;
                }
            }
        }
    }


    canvas.present();

    Ok(())
}
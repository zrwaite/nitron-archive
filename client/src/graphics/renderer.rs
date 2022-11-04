use std::collections::HashMap;

use sdl2::ttf::Font;
use specs::Join;
use specs::prelude::{ReadStorage};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture, TextureQuery};
use sdl2::rect::{Rect, Point};

use crate::processor::{ProcessorData};
use crate::assets::{TEXTURES, FONTS};

use super::get_graphics;

pub type SystemData<'a> = (
    ReadStorage<'a, ProcessorData>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &HashMap<String, Texture>,
    fonts: &HashMap<String, Font>,
    data: SystemData,
    debug: bool,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let (screen_width, screen_height) = canvas.output_size()?;

    // Test font rendering
    let texture_creator = canvas.texture_creator();

    // render a surface, and convert it to a texture bound to the canvas
    let surface = fonts[FONTS.electrolize]
        .render("Hello World!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    // canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
    // canvas.clear();

    let ideal_height = 40;

    let TextureQuery { mut width, mut height, .. } = texture.query();

    let scale = ideal_height as f32 / height as f32;

    height = (height as f32 * scale) as u32;
    width = (width as f32 * scale) as u32;

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 4;
    let target = Rect::from_center(
        Point::new(screen_width as i32/ 2, screen_height as i32 / 2),
        width - padding, 
        height - padding,
    );

    canvas.copy(&texture, None, Some(target))?;



    let processor_data = &data.0.join().next().unwrap();
    match processor_data {
        ProcessorData::Game(game) => {
            let x_scale = screen_width as f64 / game.map.width as f64;
            let y_scale = screen_height as f64 / game.map.height as f64;
        
            let graphics = get_graphics(game, x_scale, y_scale);
            
            for graphic in graphics {
                canvas.copy(&textures[&graphic.texture_key], graphic.src, graphic.dst)?;
                if debug {
                    canvas.copy(&textures[TEXTURES.debug_box], Rect::new(0,  0, 24, 24), graphic.hitbox_dst)?;
                }
            }
        }
    }
    // canvas.draw_rect(rect)
    // canvas.


    canvas.present();

    Ok(())
}
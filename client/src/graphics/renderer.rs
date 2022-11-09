use std::collections::HashMap;
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::models::HashVec;
use crate::graphics::graphic::Renderable;

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &HashMap<String, Texture>,
    fonts: &HashMap<String, Font>,
    elements: &HashVec,
    width: u32,
    height: u32,
    debug: bool,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let (screen_width, screen_height) = canvas.output_size()?;

    let x_scale = screen_width as f64 / width as f64;
    let y_scale = screen_height as f64 / height as f64;

    for element in elements.iter() {
        element.render(
            canvas,
            &textures,
            &fonts,
            x_scale,
            y_scale,
            debug
        )
        
    }

    canvas.present();

    Ok(())
}
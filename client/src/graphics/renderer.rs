use std::collections::HashMap;
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::models::{HashVec, HasId};
use crate::graphics::graphic::{Renderable,HasZIndex};

pub struct GraphicIndex {
    pub id: String,
    pub z_index: i32,
}

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &HashMap<String, Texture>,
    fonts: &HashMap<String, Font>,
    elements: &mut HashVec,
    width: u32,
    height: u32,
    debug: bool,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.clear();
    let (screen_width, screen_height) = canvas.output_size()?;

    let x_scale = screen_width as f64 / width as f64;
    let y_scale = screen_height as f64 / height as f64;


    // TODO: Hargun, make this more efficient

    let mut element_graphics: Vec<GraphicIndex> = Vec::new();
    for element in elements.iter() {
        element_graphics.push(GraphicIndex {
            id: element.id().clone(),
            z_index: element.z_index(),
        })
    }
    element_graphics.sort_by(|a, b| a.z_index.cmp(&b.z_index));

    for element_graphic in element_graphics {
        let element = elements.get(element_graphic.id).unwrap();
        element.render(
            canvas,
            textures,
            fonts,
            x_scale,
            y_scale,
            debug,
        );
    }

    canvas.present();

    Ok(())
}
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
    // let elements = &data.0.join().next().unwrap();

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

    // match processor_data {
    //     ProcessorData::Game(game) => {
    //         let graphics = get_graphics(game, x_scale, y_scale);
            
    //         for graphic in graphics {
    //             canvas.copy(&textures[&graphic.texture_key], graphic.src, graphic.dst)?;
    //             if debug {
    //                 canvas.set_draw_color(Color::RGB(255, 0, 0));
    //                 canvas.draw_rect(graphic.hitbox_dst)?;
    //             }
    //         }
    //     }
    //     ProcessorData::None => {}
    // }

    // for ui_element in ui_elements {
        // render_ui_element(
        //     canvas,
        //     &ui_element,
        //     &textures,
        //     &fonts,
        //     x_scale,
        //     y_scale,
        // )?;
    // }

    canvas.present();

    Ok(())
}
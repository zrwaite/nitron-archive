use std::collections::HashMap;

use sdl2::ttf::Font;
use specs::Join;
use specs::prelude::{ReadStorage};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::processor::{ProcessorData};
use crate::ui::UIElement;

use super::ui_graphics::render_ui_element;
use super::{get_graphics};

pub type SystemData<'a> = (
    ReadStorage<'a, ProcessorData>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &HashMap<String, Texture>,
    fonts: &HashMap<String, Font>,
    ui_elements: &Vec<UIElement>,
    data: SystemData,
    width: u32,
    height: u32,
    debug: bool,
) -> Result<(), String> {
    let processor_data = &data.0.join().next().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let (screen_width, screen_height) = canvas.output_size()?;

    let x_scale = screen_width as f64 / width as f64;
    let y_scale = screen_height as f64 / height as f64;

    match processor_data {
        ProcessorData::Game(game) => {
            let graphics = get_graphics(game, x_scale, y_scale);
            
            for graphic in graphics {
                canvas.copy(&textures[&graphic.texture_key], graphic.src, graphic.dst)?;
                if debug {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    canvas.draw_rect(graphic.hitbox_dst)?;
                }
            }
        }
        ProcessorData::None => {}
    }

    for ui_element in ui_elements {
        render_ui_element(
            canvas,
            &ui_element,
            &textures,
            &fonts,
            x_scale,
            y_scale,
        )?;
    }

    canvas.present();

    Ok(())
}
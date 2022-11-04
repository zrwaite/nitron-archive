use std::collections::HashMap;

use sdl2::ttf::Font;
use specs::Join;
use specs::prelude::{ReadStorage};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture, TextureQuery};
use sdl2::rect::{Rect, Point};

use crate::processor::{ProcessorData};
use crate::assets::{TEXTURES};
use crate::ui::UIElement;

use super::{get_graphics, scale, scale_u};

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
                    canvas.copy(&textures[TEXTURES.debug_box], Rect::new(0,  0, 24, 24), graphic.hitbox_dst)?;
                }
            }
        }
    }
    // canvas.draw_rect(rect)
        
    // font rendering
    let texture_creator = canvas.texture_creator();

    for ui_element in ui_elements {
        match ui_element {
            UIElement::Text(text_element) => {
                let font = &fonts[&text_element.font];
                let surface = font.render(&text_element.text)
                    .blended(text_element.font_color)
                    .map_err(|e| e.to_string())?;
                let texture = texture_creator.create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;
                let TextureQuery { width, height, .. } = texture.query();
                let x = scale(text_element.styles.x, x_scale);
                let y = scale(text_element.styles.y, y_scale);
                let scale = text_element.styles.height as f64 / height as f64;
                // add padding and proper scaling?
                let dst = Rect::from_center(
                    Point::new(x, y), 
                    scale_u(width as i32, scale * x_scale),
                    scale_u(height as i32, scale * y_scale),
                );
                canvas.copy(&texture, None, dst)?;
            }
            UIElement::Image(image_element) => {
                let texture = &textures[&image_element.image];
                let TextureQuery { width, height, .. } = texture.query();
                let x = image_element.styles.x;
                let y = image_element.styles.y;
                let dst = Rect::from_center(Point::new(x as i32, y as i32), width, height);
                canvas.copy(&texture, None, dst)?;
            }
        }
    }

    canvas.present();

    Ok(())
}
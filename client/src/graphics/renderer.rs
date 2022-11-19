use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::{GAME_WIDTH, GAME_HEIGHT};
use crate::assets::TEXTURES;
use crate::entities::{HashVec, HasId, GameEntity};
use crate::graphics::graphic::{Renderable,HasZIndex};
use crate::physics::InteractionHitbox;
use crate::utils::{Vector2, Vector4};

use super::{scale, scale_u, Graphic};

pub struct GraphicIndex {
    pub id: String,
    pub z_index: i32,
}

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &HashMap<String, Texture>,
    fonts: &HashMap<String, Font>,
    elements: &mut HashVec,
    map_width: u32,
	map_height: u32,
    debug: bool,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.clear();
    let (screen_width, screen_height) = canvas.output_size()?;

    


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
        let (x_scale, y_scale) = match element {
            GameEntity::Box(_) => {
                let x_scale = screen_width as f64 / GAME_WIDTH as f64;
                let y_scale = screen_height as f64 / GAME_HEIGHT as f64;
                (x_scale, y_scale)
            }
            _ => {
                let x_scale = screen_width as f64 / map_width as f64;
                let y_scale = screen_height as f64 / map_height as f64;
                (x_scale, y_scale)
            }
        };
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

pub fn simple_render( 
    canvas: &mut WindowCanvas,
    pos: Vector2,
    texture_key: String,
    display_size: Vector2,
    frame: Rect,
    hitbox: Vector4,
    interaction_hitbox: InteractionHitbox,
    textures: &HashMap<String, Texture>,
    _fonts: &HashMap<String, Font>,
    x_scale: f64,
    y_scale: f64,
    debug: bool
) {
    let hitbox_rect = hitbox.get_scaled(x_scale, y_scale).to_rect();
    let screen_rect = Rect::from_center(
        (
            scale(pos.x, x_scale),
            scale(pos.y, y_scale),
        ),
        scale_u(display_size.x, x_scale),
        scale_u(display_size.y, y_scale),
    );

    let graphic = Graphic {
        texture_key: texture_key.to_string(),
        src: frame,
        dst: screen_rect,
        hitbox_dst: hitbox_rect,
        radius_dst: interaction_hitbox.to_scaled_rect(x_scale, y_scale),
        z_index: hitbox.y,
    };
    canvas.copy(&textures[&graphic.texture_key], graphic.src, graphic.dst).unwrap();
    if debug {
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.draw_rect(graphic.hitbox_dst).unwrap();
        canvas.copy(&textures[TEXTURES.circle], Rect::new(0, 0, 32, 32), graphic.radius_dst).unwrap();
    }
}
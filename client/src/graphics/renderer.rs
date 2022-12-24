use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::render::{WindowCanvas, Texture};

use crate::physics::InteractionHitbox;
use crate::utils::{Vector2, Vector4};




// pub fn render(
//     canvas: &mut WindowCanvas,
//     textures: &HashMap<String, Texture>,
//     fonts: &HashMap<String, Font>,
//     entity_store: &mut EntityStore,
//     map_width: u32,
// 	map_height: u32,
//     debug: bool,
// ) -> Result<(), String> {
//     canvas.set_draw_color(Color::RGB(100, 100, 100));
    
// }

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
    
}
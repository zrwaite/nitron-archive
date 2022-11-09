use std::collections::HashMap;

use sdl2::{rect::Rect, render::{WindowCanvas, Texture}, ttf::Font};

pub fn scale(n: i32, scale: f64) -> i32 {
	(n as f64* scale) as i32
}

pub fn scale_u(n: i32, scale: f64) -> u32 {
	(n as f64 * scale) as u32
}

pub trait Renderable {
	fn render(&self, 
		canvas: &mut WindowCanvas,
		textures: &HashMap<String, Texture>,
    	fonts: &HashMap<String, Font>,
		x_scale: f64,
		y_scale: f64,
		debug: bool
	);
}

pub struct Graphic {
	pub texture_key: String,
	pub src: Rect, 
	pub dst: Rect,
	pub hitbox_dst: Rect,
	pub z_index: i32,
}

// pub fn get_graphics(data: &GameData, x_scale: f64, y_scale: f64) -> Vec<Graphic> {
// 	let mut graphics = Vec::new();
// 	let player = &data.player;
//     for obstacle in &data.map.static_obstacles {
// 		let texture_key = &obstacle.display.texture_key;
//         let current_frame = obstacle.frame.region;
// 		let hitbox = obstacle.hitbox();
//         let screen_rect = Rect::from_center(
//             Point::new(
// 				scale(obstacle.pos.x, x_scale),
// 				scale(obstacle.pos.y, y_scale),
//             ),
// 			scale_u(obstacle.display.size.x, x_scale),
// 			scale_u(obstacle.display.size.y, y_scale),
//         );
// 		graphics.push(Graphic {
// 			texture_key: texture_key.to_string(),
// 			src: current_frame,
// 			dst: screen_rect,
// 			hitbox_dst: hitbox.get_scaled(x_scale, y_scale).to_rect(),
// 			z_index: hitbox.y,
// 		});
// 	}

// 	let texture_key = &data.player.display.texture_key;
//     let current_frame = player.animator.current_frame.region;
// 	let hitbox = player.hitbox();
//     let screen_rect = Rect::from_center(
//         Point::new(
// 			scale(player.pos.x, x_scale),
// 			scale(player.pos.y, y_scale),
//         ),
// 		scale_u(player.display.size.x, x_scale),
// 		scale_u(player.display.size.y, y_scale),
//     );

// 	graphics.push(Graphic {
// 		texture_key: texture_key.to_string(),
// 		src: current_frame,
// 		dst: screen_rect,
// 		hitbox_dst: hitbox.get_scaled(x_scale, y_scale).to_rect(),
// 		z_index: hitbox.y,
// 	});

// 	// TODO make this more efficient: Don't sort very loop
// 	graphics.sort_by(|a, b| a.z_index.partial_cmp(&b.z_index).unwrap());

// 	graphics
// }
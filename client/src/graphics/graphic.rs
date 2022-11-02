use sdl2::rect::{Rect, Point};

use crate::processor::{GameData};

pub struct Graphic {
	pub texture_key: String,
	pub src: Rect, 
	pub dst: Rect,
	pub hitbox_dst: Rect,
	pub z_index: i32,
}

pub fn get_graphics(data: &GameData, x_scale: f64, y_scale: f64) -> Vec<Graphic> {
	let mut graphics = Vec::new();
	let player = &data.player;
    for obstacle in &data.map.static_obstacles {
		let texture_key = &obstacle.display.texture_key;
        let current_frame = obstacle.frame.region;
		let hitbox = obstacle.hitbox();
        let screen_position = Point::new((obstacle.pos.x as f64 * x_scale) as i32 , (obstacle.pos.y as f64 * y_scale) as i32);
        let screen_rect = Rect::from_center(
            Point::new(
                screen_position.x(),
                screen_position.y(),
            ),
            (obstacle.display.size.x as f64 * x_scale) as u32,
            (obstacle.display.size.y as f64* y_scale) as u32,
        );
		graphics.push(Graphic {
			texture_key: texture_key.to_string(),
			src: current_frame,
			dst: screen_rect,
			hitbox_dst: hitbox.get_scaled(x_scale, y_scale).to_rect(),
			z_index: hitbox.y,
		});
	}

	let texture_key = &data.player.display.texture_key;
    let current_frame = player.animator.current_frame.region;
	let hitbox = player.hitbox();
    let screen_position = Point::new((player.pos.x as f64 * x_scale) as i32 , (player.pos.y as f64 * y_scale) as i32);
    let screen_rect = Rect::from_center(
        Point::new(
            screen_position.x(),
            screen_position.y(),
        ),
        (player.display.size.x as f64 * x_scale) as u32,
        (player.display.size.y as f64* y_scale) as u32,
    );

	graphics.push(Graphic {
		texture_key: texture_key.to_string(),
		src: current_frame,
		dst: screen_rect,
		hitbox_dst: hitbox.get_scaled(x_scale, y_scale).to_rect(),
		z_index: hitbox.y,
	});

	// TODO make this more efficient: Don't sort very loop
	graphics.sort_by(|a, b| a.z_index.partial_cmp(&b.z_index).unwrap());

	graphics
}
use crate::utils::{Vector4};
use crate::entities::player::Player;

#[derive(Debug, PartialEq)]
pub enum CollisionResult {
	Top,
	Bottom,
	Left,
	Right,
	None
}

pub fn detect_collision(hitbox_a: &Vector4, hitbox_b: &Vector4) -> CollisionResult {
	let hitbox_h_sum = hitbox_a.h / 2 + hitbox_b.h / 2;
	let hitbox_w_sum = hitbox_a.w / 2 + hitbox_b.w / 2;

	if 
		(hitbox_a.y + hitbox_h_sum >= hitbox_b.y) && 
		(hitbox_a.y <= hitbox_b.y + hitbox_h_sum) && 
		(hitbox_a.x + hitbox_w_sum >= hitbox_b.x) && 
		(hitbox_a.x <= hitbox_b.x + hitbox_w_sum) 
	{
		let dl = (hitbox_a.x + hitbox_w_sum - hitbox_b.x) as f32; 
		let dr = (hitbox_b.x + hitbox_w_sum - hitbox_a.x) as f32; 
		let dt = (hitbox_a.y + hitbox_h_sum - hitbox_b.y) as f32; 
		let db = (hitbox_b.y + hitbox_h_sum - hitbox_a.y) as f32; 

		if dl < dt && dl < db {
			return CollisionResult::Left;
		} else if dr < dt && dr < db {
			return CollisionResult::Right;
		} else if dt < db {
			return CollisionResult::Top;
		} else {
			return CollisionResult::Bottom;
		}
	}

	return CollisionResult::None;

}

pub fn player_static_obstacle_collision(player: &mut Player, obstacle_hitbox: &Vector4) {
	let hitbox = player.hitbox();
	match detect_collision(&hitbox, obstacle_hitbox) {
		CollisionResult::Top => {
			player.set_y_by_hitbox(obstacle_hitbox.y - obstacle_hitbox.h / 2 - hitbox.h / 2);
		},
		CollisionResult::Bottom => {
			player.set_y_by_hitbox(obstacle_hitbox.y + obstacle_hitbox.h / 2 + hitbox.h / 2);
		},
		CollisionResult::Left => {	
			player.set_x_by_hitbox(obstacle_hitbox.x - obstacle_hitbox.w / 2 - hitbox.w / 2);
		},
		CollisionResult::Right => {
			player.set_x_by_hitbox(obstacle_hitbox.x + obstacle_hitbox.w / 2 + hitbox.w / 2);
		},
		CollisionResult::None => {}
	}
}
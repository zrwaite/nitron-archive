use crate::entities::{StaticObstacle, Npc};
use crate::utils::{Vector4};
use crate::entities::player::Player;

use super::{InteractionHitbox};

pub enum CollisionObject<'a> {
	Static(&'a mut StaticObstacle),
	Dynamic(&'a mut Npc),
}
impl CollisionObject<'_> {
	pub fn hitbox(&self) -> Vector4 {
		match self {
			CollisionObject::Static(obstacle) => obstacle.hitbox(),
			CollisionObject::Dynamic(npc) => npc.hitbox(),
		}
	}
	pub fn interaction_hitbox(&self) -> InteractionHitbox {
		match self {
			CollisionObject::Static(obstacle) => obstacle.interaction_hitbox(),
			CollisionObject::Dynamic(npc) => npc.interaction_hitbox(),
		}
	}
}

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

pub fn player_obstacle_collision(player: &mut Player, obstacle_hitbox: &Vector4) {
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

pub fn player_obstacle_interaction(player: &InteractionHitbox, obstacle: &InteractionHitbox) -> bool {
	let distance = (((player.x - obstacle.x).pow(2) + (player.y - obstacle.y).pow(2)) as f32).sqrt();
	if distance < (obstacle.r + player.r) as f32 {
		return true;
	}
	return false;
}
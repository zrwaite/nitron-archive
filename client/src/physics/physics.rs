use crate::entities::Player;
use crate::entities::{GameEntity,HashVec};
use crate::engine::EngineState;

use super::collision::{player_static_obstacle_collision, CollisionObject};
pub fn run_physics(game_entities: &mut HashVec, engine_state: &mut EngineState) {  

    match engine_state {
        EngineState::Playing(game) => {
            let mut collision_objects: Vec<CollisionObject> = Vec::new();
            let mut player_option: Option<&mut Player> = None;
            for entity in game_entities.iter_mut() {
                match entity {
                    GameEntity::StaticObstacle(obstacle) => {
                        collision_objects.push(CollisionObject::Static(obstacle));
                    }
                    GameEntity::Npc(obj) => {
                        collision_objects.push(CollisionObject::Dynamic(obj));
                    }
                    GameEntity::Player(obj) => {
                        player_option = Some(obj);
                    }
                    _ => {}
                }
            }
            let player = match player_option {
                Some(player) => player,
                None => panic!("No player found in game entities"),
            };

            for obstacle in collision_objects.iter_mut() {
                //TODO Make this more efficient: Quad tree? 
                // collision detection
                player_static_obstacle_collision(player, &mut obstacle.hitbox());
            }

            player.pos.offset(player.vel.x, player.vel.y);
            let hitbox = player.hitbox();
            if hitbox.x < hitbox.w / 2  {
                player.set_x_by_hitbox(hitbox.w / 2);
            }
            if hitbox.x + hitbox.w / 2 > game.block().width as i32 {
                player.set_x_by_hitbox(game.block().width as i32 - hitbox.w / 2);
            }
            if hitbox.y < hitbox.h / 2 {
                player.set_y_by_hitbox(hitbox.h / 2);
            }
            if hitbox.y + hitbox.h /2 > game.block().height as i32 {
                player.set_y_by_hitbox(game.block().height as i32 - hitbox.h / 2);
            }
        }
        _ => {}
    }
}
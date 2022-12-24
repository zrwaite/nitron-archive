use crate::{sprites::Player, entity_lib::EntityStore};
use crate::engine::EngineState;
use crate::entity_lib::UIBox;
use std::collections::HashMap;

use super::collision::{player_obstacle_collision, CollisionObject, player_obstacle_interaction};
pub fn run_physics(game_entities: &mut EntityStore, engine_state: &mut EngineState) {  

    match engine_state {
        EngineState::Playing(game) => {
            let mut collision_objects: Vec<CollisionObject> = Vec::new();
            let mut player_option: Option<&mut Player> = None;
            let mut ui_box_map: HashMap<String, &mut UIBox> = HashMap::new();
            for entity in game_entities.iter_mut() {
                // match entity {
                    // GameEntity::StaticObstacle(obstacle) => {
                    //     //TODO Quad tree push
                    //     collision_objects.push(CollisionObject::Static(obstacle));
                    // }
                    // GameEntity::Npc(obj) => {
                    //     //TODO Quad tree push
                    //     collision_objects.push(CollisionObject::Dynamic(obj));
                    // }
                    // GameEntity::Player(obj) => {
                    //     //TODO Quad tree push
                    //     player_option = Some(obj);
                    // }
                // }
            }
            let player = match player_option {
                Some(player) => player,
                None => panic!("No player found in game entities"),
            };

            for obstacle in collision_objects.iter_mut() {
                //TODO Make this more efficient: Quad tree? 
                // collision detection
                player_obstacle_collision(player, &mut obstacle.hitbox());
                if player_obstacle_interaction(&player.interaction_hitbox(), &obstacle.interaction_hitbox()) {
                    match obstacle {
                        CollisionObject::Static(obstacle) => {
                            obstacle.enable_player_interaction();
                        }
                        CollisionObject::Dynamic(npc) => {
                            // npc.enable_player_interaction(&mut ui_box_map);
                        }
                    }
                } else {
                    match obstacle {
                        CollisionObject::Static(obstacle) => {
                            obstacle.disable_player_interaction();
                        }
                        CollisionObject::Dynamic(npc) => {
                            // npc.disable_player_interaction(&mut ui_box_map);
                        }
                    }
                }
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
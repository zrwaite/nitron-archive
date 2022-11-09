use specs::prelude::{System, WriteStorage};
use specs::Join;
use crate::models::HashVec;
use crate::processor::EngineState;

use super::collision;

pub struct EngineStateConverter {}

impl<'a> System<'a> for EngineStateConverter {
    type SystemData = (
        WriteStorage<'a, HashVec>,
        WriteStorage<'a, EngineState>
    );

    fn run(&mut self, mut data: Self::SystemData) {      
        let (game_entities, engine_state) = (&mut data.0, &mut data.1).join().next().unwrap();

        match engine_state {
            EngineState::Playing(game) => {
        		let player = game_entities.get(game.player_id.clone()).unwrap().mut_unwrap_player();

        
                // for obstacle in game.map.static_obstacles.iter_mut() {
                //     //TODO Make this more efficient: Quad tree? 
                //     // collision detection
                //     collision::player_static_obstacle_collision(player, &mut obstacle.hitbox());
                // }
                player.pos.offset(player.vel.x, player.vel.y);
                let hitbox = player.hitbox();
                if hitbox.x < hitbox.w / 2  {
                    player.set_x_by_hitbox(hitbox.w / 2);
                }
                if hitbox.x + hitbox.w / 2 > game.map.width as i32 {
                    player.set_x_by_hitbox(game.map.width as i32 - hitbox.w / 2);
                }
                if hitbox.y < hitbox.h / 2 {
                    player.set_y_by_hitbox(hitbox.h / 2);
                }
                if hitbox.y + hitbox.h /2 > game.map.height as i32 {
                    player.set_y_by_hitbox(game.map.height as i32 - hitbox.h / 2);
                }
            }
            _ => {}
        }
    }
}


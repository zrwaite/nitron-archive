use crate::utils::{Direction};
use crate::input::{KeyTracker};
use crate::entities::HashVec;
use crate::engine::EngineState;

const PLAYER_MOVEMENT_SPEED: i32 = 2;

pub fn run_controller(presses: &mut KeyTracker, game_entities: &mut HashVec, engine_state: &mut EngineState) {  
    match engine_state {
        EngineState::Playing(game) => {
            let player = game_entities.get(game.player_id.clone()).unwrap().mut_unwrap_player();

            if presses.down {
                player.vel.y = PLAYER_MOVEMENT_SPEED;
                player.display.direction = Direction::Down;
            } else if presses.up {
                player.vel.y = - PLAYER_MOVEMENT_SPEED;
                player.display.direction = Direction::Up;
            } else {
                player.vel.y = 0;
            }
            if presses.left {
                player.vel.x = - PLAYER_MOVEMENT_SPEED;
                player.display.direction = Direction::Left;
            } else if presses.right {
                player.vel.x = PLAYER_MOVEMENT_SPEED;
                player.display.direction = Direction::Right;
            } else {
                player.vel.x = 0;
            }   
        }
        _ => {}
    }
}
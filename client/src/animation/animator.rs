use crate::entities::HashVec;
use crate::engine::EngineState;

pub fn run_animator(game_entities: &mut HashVec, engine_state: &mut EngineState) {  
    match engine_state {
        EngineState::Playing(game) => {
            let player = game_entities.get(game.player_id.clone()).unwrap().mut_unwrap_player();
            let moving = player.vel.x != 0 || player.vel.y != 0;
            player.animator.load_frame(player.display.direction.clone(), moving)
        }
        _ => {}
    }
}


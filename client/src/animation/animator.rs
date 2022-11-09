use crate::components::Direction; 
use crate::models::HashVec;
use crate::engine::EngineState;

pub fn run_animator(game_entities: &mut HashVec, engine_state: &mut EngineState) {  
    match engine_state {
        EngineState::Playing(game) => {
            let player = game_entities.get(game.player_id.clone()).unwrap().mut_unwrap_player();
            let moving = player.vel.x != 0 || player.vel.y != 0;
            player.animator.load_frame(player.display.direction.clone(), moving)
            // let movement_animation = &mut player.animator;   
            // let player_moving = player.vel.x != 0 || player.vel.y != 0;
            // movement_animation.current_frame = frames[frame_index].clone(); 
        }
        _ => {}
    }
}


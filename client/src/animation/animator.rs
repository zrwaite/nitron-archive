use crate::components::Direction; 
use crate::models::HashVec;
use crate::engine::EngineState;

pub fn run_animator(game_entities: &mut HashVec, engine_state: &mut EngineState) {  
    match engine_state {
        EngineState::Playing(game) => {
            let player = game_entities.get(game.player_id.clone()).unwrap().mut_unwrap_player();
            let movement_animation = &mut player.animator;   
            if player.vel.x == 0 && player.vel.y ==0 {
                return;
            }
            let frames = match player.display.direction {
                Direction::Left => &movement_animation.left_frames,
                Direction::Right => &movement_animation.right_frames,
                Direction::Up => &movement_animation.up_frames,
                Direction::Down => &movement_animation.down_frames,
            };
            movement_animation.current_timer = (movement_animation.current_timer + 1) % movement_animation.max_timer;
            if movement_animation.current_timer == 0 {
                movement_animation.current_frame_index = (movement_animation.current_frame_index + 1) % 4;
            }
            let frame_index = match movement_animation.current_frame_index {
                0|2 => 1,
                1 => 2,
                3 => 0,
                _ => 1
            };
            movement_animation.current_frame = frames[frame_index].clone(); 
        }
        _ => {}
    }
}


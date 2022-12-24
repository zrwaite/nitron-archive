use crate::{engine::EngineState, entity_lib::EntityStore};

pub fn run_animator(entity_store: &mut EntityStore, engine_state: &mut EngineState) {  
    match engine_state {
        EngineState::Playing(game) => {
            let moving = game.player.vel.x != 0 || game.player.vel.y != 0;
            game.player.animator.load_frame(game.player.display.direction.clone(), moving)
        }
        _ => {}
    }
}


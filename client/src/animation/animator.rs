use crate::components::{Direction};
use crate::processor::{ProcessorData};
use specs::prelude::{System, WriteStorage, Join};

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, ProcessorData>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let processor_data = (&mut data.0).join().next().unwrap();
        match processor_data {
            ProcessorData::Game(game) => {
                let player = &mut game.player;
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
            ProcessorData::None => {}
        }
    }
}


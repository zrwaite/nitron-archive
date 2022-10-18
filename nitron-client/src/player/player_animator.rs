use specs_derive::Component;
use specs::prelude::{Component, VecStorage};
use sdl2::rect::Rect;

use crate::{animation::AnimationFrame, components::Direction};


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct PlayerAnimator {
    // The current frame in the animation of the direction this entity is moving in
    pub current_frame_index: usize,
	pub current_frame: AnimationFrame,
    pub current_timer: u32,
    pub max_timer: u32,
    pub up_frames: Vec<AnimationFrame>,
    pub down_frames: Vec<AnimationFrame>,
    pub left_frames: Vec<AnimationFrame>,
    pub right_frames: Vec<AnimationFrame>,
}

impl PlayerAnimator {
    pub fn new() -> Self {
		let top_left_frame = Rect::new(0, 0, 26, 36);
        PlayerAnimator {
            current_frame_index: 0,
            up_frames: PlayerAnimator::get_animation_frame(top_left_frame, Direction::Up),
            down_frames: PlayerAnimator::get_animation_frame(top_left_frame, Direction::Down),
            left_frames: PlayerAnimator::get_animation_frame(top_left_frame, Direction::Left),
            right_frames: PlayerAnimator::get_animation_frame(top_left_frame, Direction::Right),
			current_frame: AnimationFrame {
				region: top_left_frame
			},
            current_timer: 0,
            max_timer: 8,
        }
    }
    pub fn get_animation_frame(top_left_frame: Rect, direction: Direction) -> Vec<AnimationFrame> {
        let (frame_width, frame_height) = top_left_frame.size();
        let y_offset = top_left_frame.y() + frame_height as i32 * PlayerAnimator::direction_spritesheet_row(direction);
    
        let mut frames = Vec::new();
        for i in 0..3 {
            frames.push(AnimationFrame {
                region: Rect::new(
                    top_left_frame.x() + frame_width as i32 * i,
                    y_offset,
                    frame_width,
                    frame_height,
                ),
            })
        }
    
        frames
    }
    fn direction_spritesheet_row( direction: Direction) -> i32 {
        use self::Direction::*;
        match direction {
            Up => 3,
            Down => 0,
            Left => 1,
            Right => 2,
        }
    }
}
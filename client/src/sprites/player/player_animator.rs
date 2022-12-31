use specs_derive::Component;
use specs::prelude::{Component, VecStorage};
use sdl2::rect::Rect;

use crate::{utils::Direction};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct PlayerAnimator {
    // The current frame in the animation of the direction this entity is moving in
    pub current_frame: Rect,    
    current_frame_index: u32,
    current_timer: u32,
    max_timer: u32,
    up_frames: Vec<Rect>,
    down_frames: Vec<Rect>,
    left_frames: Vec<Rect>,
    right_frames: Vec<Rect>,
}

impl PlayerAnimator {
    pub fn new() -> Self {
		let top_left_frame = Rect::new(0, 0, 16, 20);
        PlayerAnimator {
            current_frame: top_left_frame,
            current_frame_index: 0,
            up_frames: PlayerAnimator::get_animation_frame(top_left_frame, Direction::Up),
            down_frames: PlayerAnimator::get_animation_frame(top_left_frame, Direction::Down),
            left_frames: PlayerAnimator::get_animation_frame(top_left_frame, Direction::Left),
            right_frames: PlayerAnimator::get_animation_frame(top_left_frame, Direction::Right),
            current_timer: 0,
            max_timer: 8,
        }
    }
    fn get_animation_frame(top_left_frame: Rect, direction: Direction) -> Vec<Rect> {
        let (frame_width, frame_height) = top_left_frame.size();
        let y_offset = top_left_frame.y() + frame_height as i32 * PlayerAnimator::direction_spritesheet_row(direction);
        
        let mut frames = Vec::new();
        for i in 0..3 {
            frames.push(Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ))
        }
    
        frames
    }
    fn direction_spritesheet_row(direction: Direction) -> i32 {
        use self::Direction::*;
        match direction {
            Down => 0,
            Right => 1,
            Left => 2,
            Up => 3,
        }
    }
    pub fn load_frame(&mut self, direction: Direction, moving: bool) {
        let frames = match direction {
            Direction::Left => &self.left_frames,
            Direction::Right => &self.right_frames,
            Direction::Up => &self.up_frames,
            Direction::Down => &self.down_frames,
        };

        if moving {
            self.current_timer = (self.current_timer + 1) % self.max_timer;
            if self.current_timer == 0 {
                self.current_frame_index = (self.current_frame_index + 1) % 2;
            }
            let frame_index = match self.current_frame_index {
                0 => 1,
                1 => 2,
                _ => panic!("Invalid frame index"),
            };
            self.current_frame = frames[frame_index]; 
        } else {
            self.current_frame = frames[0];
        }
    }
}
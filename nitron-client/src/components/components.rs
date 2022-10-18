use specs_derive::Component;
use specs::prelude::{Component, VecStorage, NullStorage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// #[derive(Component, Debug, Clone, Copy)]
// #[storage(VecStorage)]
// pub struct Vector2F {
//     pub x: f32,
//     pub y: f32,
// }

// impl Vector2F {
//     pub fn new(x: f32, y: f32) -> Self {
//         Self { x, y }
//     }
// }



#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }
    pub fn offset(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
    pub fn get_scaled(&self, scale: f32) -> Vector2 {
        Vector2 {
            x: (self.x as f32 * scale) as i32,
            y: (self.y as f32 * scale) as i32,
        }
    }
}


#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component, Debug, Copy, Clone)]
#[storage(VecStorage)]
pub struct KeyTracker {
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool
}
impl KeyTracker {
    pub fn new () -> KeyTracker {
        KeyTracker {
            up: false,
            down: false,
            right: false,
            left: false
        }
    }
}
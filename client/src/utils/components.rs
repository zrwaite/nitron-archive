use serde::{Deserialize, Serialize};
use specs_derive::Component;
use specs::prelude::{Component, VecStorage};
use std::ops;
use sdl2::rect::{Rect, Point};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2 { x, y }
    }
    pub fn offset(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}


#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vector3 { x, y, z }
    }
    pub fn to_vector2(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

pub struct Vector4 {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Vector4 {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Vector4 { x, y, w, h }
    }
    pub fn get_scaled(&self, x_scale: f64, y_scale: f64) -> Vector4 {
        Vector4 {
            x: (self.x as f64 * x_scale) as i32,
            y: (self.y as f64 * y_scale) as i32,
            w: (self.w as f64 * x_scale) as i32,
            h: (self.h as f64 * y_scale) as i32,
        }
    }
    pub fn to_rect(&self) -> Rect {
        Rect::from_center(Point::new(
            self.x, self.y,
        ), self.w as u32, self.h as u32)
    }
}

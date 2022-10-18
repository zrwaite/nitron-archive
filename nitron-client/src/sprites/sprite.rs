use specs_derive::Component;
use specs::prelude::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    // pub direction: Direction,
	// pub movement_animation: MovementAnimation,
    // pub texture_key: String,
	// pub pos: Vector2,
	// pub vel: Vector2,
    // pub size: Vector2,
}

impl Sprite {
	// pub fn new(
    //     pos: Vector2, 
    //     size: Vector2, 
    //     movement_animation: MovementAnimation, 
    //     texture_key: String,
    // ) -> Self {     
	// 	Sprite {
	// 		direction: Direction::Down,
	// 		pos,
    //         texture_key,
	// 		movement_animation,
	// 		vel: Vector2::new(0, 0),
    //         size
	// 	}
	// }
}

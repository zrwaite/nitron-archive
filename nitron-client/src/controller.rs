

use specs::prelude::{ReadExpect, ReadStorage, WriteStorage, Join, System};

use crate::game::Game;
use crate::components::{KeyTracker, KeyboardControlled, Direction};

const PLAYER_MOVEMENT_SPEED: i32 = 2;

pub struct Controller;

impl<'a> System<'a> for Controller {
    type SystemData = (
        ReadExpect<'a, KeyTracker>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Game>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let presses = &*data.0;
        for ( _, game) in (&data.1, &mut data.2).join() {
            let player = &mut game.player;
            if presses.down {
                player.vel.y = PLAYER_MOVEMENT_SPEED;
                player.display.direction = Direction::Down;
            } else if presses.up {
                player.vel.y = - PLAYER_MOVEMENT_SPEED;
                player.display.direction = Direction::Up;
            } else {
                player.vel.y = 0;
            }
            if presses.left {
                player.vel.x = - PLAYER_MOVEMENT_SPEED;
                player.display.direction = Direction::Left;
            } else if presses.right {
                player.vel.x = PLAYER_MOVEMENT_SPEED;
                player.display.direction = Direction::Right;
            } else {
                player.vel.x = 0;
            }
        }   
    }
}

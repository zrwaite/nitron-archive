use specs::prelude::{System, WriteStorage, Join};

use crate::{game::Game};

pub struct Physics {}

impl<'a> System<'a> for Physics {
    type SystemData = (
        WriteStorage<'a, Game>, 
    );

    fn run(&mut self, mut data: Self::SystemData) {      
        for game in (&mut data.0).join() {
            let player = &mut game.player;
            player.pos.offset(player.vel.x, player.vel.y);
            if player.pos.x < player.hitbox.x {
                player.pos.x = player.hitbox.x;
            }
            if player.pos.x + player.hitbox.x > game.map.width as i32 {
                player.pos.x = game.map.width as i32 - player.hitbox.x;
            }
            if player.pos.y < player.hitbox.y {
                player.pos.y = player.hitbox.y;
            }
            if player.pos.y + player.hitbox.y > game.map.height as i32 {
                player.pos.y = game.map.height as i32 - player.hitbox.y;
            }
        }
    }
}


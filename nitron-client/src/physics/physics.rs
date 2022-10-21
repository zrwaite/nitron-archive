use specs::prelude::{System, WriteStorage, Join};

use crate::{game::Game};

use super::collision;

pub struct Physics {}

impl<'a> System<'a> for Physics {
    type SystemData = (
        WriteStorage<'a, Game>, 
    );

    fn run(&mut self, mut data: Self::SystemData) {      
        let game = (&mut data.0).join().next().unwrap();
        let player = &mut game.player;
        for obstacle in game.map.static_obstacles.iter_mut() {
            // collision detection
            collision::player_static_obstacle_collision(player, &mut obstacle.hitbox());
        }
        player.pos.offset(player.vel.x, player.vel.y);
        let hitbox = player.hitbox();
        if hitbox.x < hitbox.w / 2  {
            player.set_x_by_hitbox(hitbox.w / 2);
        }
        if hitbox.x + hitbox.w / 2 > game.map.width as i32 {
            player.set_x_by_hitbox(game.map.width as i32 - hitbox.w / 2);
        }
        if hitbox.y < hitbox.h / 2 {
            player.set_y_by_hitbox(hitbox.h / 2);
        }
        if hitbox.y + hitbox.h /2 > game.map.height as i32 {
            player.set_y_by_hitbox(game.map.height as i32 - hitbox.h / 2);
        }
    }
}


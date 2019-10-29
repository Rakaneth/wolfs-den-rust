use specs::prelude::*;
use super::utils::{clamp};
use super::components::{Position, Player};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = clamp(pos.x + dx, 0, 100);
        pos.y = clamp(pos.y + dy, 0, 40);
    }
}
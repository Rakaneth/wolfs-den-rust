use rltk::{VirtualKeyCode, Rltk, Console, RGB};
use super::player::{try_move_player};
use super::map::{Map, TileType};
use super::{State};


pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some (key) => match key {
            VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad9 => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Numpad3 => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad1 => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Numpad7 => try_move_player(-1, -1, &mut gs.ecs),
            _ => {}
        }
    }
}

pub fn draw_map(map: &Map, ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.tiles.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_u8(121, 121, 121), RGB::named(rltk::BLACK), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_u8(191, 191, 191), RGB::named(rltk::BLACK), rltk::to_cp437('#'));
            }
            TileType::Door(false) => {
                ctx.set(x, y, RGB::from_u8(191, 121, 101), RGB::named(rltk::BLACK), rltk::to_cp437('+'));
            }
            TileType::Door(true) => {
                ctx.set(x, y, RGB::from_u8(191, 121, 101), RGB::named(rltk::BLACK), rltk::to_cp437('/'));
            }
            TileType::NullTile => {}
        }

        x += 1;
        if x >= map.width {
            x = 0;
            y += 1;
        }
        if y >= 40 {
            break;
        }
    }
}
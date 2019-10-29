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
    let mut fg: RGB = RGB::named(rltk::BLACK);
    let mut glyph = '\x00';
    for tile in map.tiles.iter() {
        match tile {
            TileType::Floor => {
                fg = RGB::from_u8(121, 121, 121);
                glyph = '.';
            }
            TileType::Wall => {
                fg = RGB::from_u8(191, 191, 191);
                glyph = '#';
            }
            TileType::Door(false) => {
                fg = RGB::from_u8(191, 121, 101);
                glyph = '+';
            }
            TileType::Door(true) => {
                fg = RGB::from_u8(191, 121, 101);
                glyph = '/';
            }
            TileType::NullTile => {}
        }

        if glyph > '\x00' {
            ctx.set(x, y, fg, RGB::named(rltk::BLACK), rltk::to_cp437(glyph));
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
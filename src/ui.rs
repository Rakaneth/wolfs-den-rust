use rltk::{VirtualKeyCode, Rltk, Console, RGB};
use super::player::{try_move_player};
use super::map::{Map, TileType};
use super::{State};
use super::components::*;
use specs::prelude::*;
use super::utils::*;

pub trait Screen {
    fn name(&self) -> &str;
    fn render(&self, ecs: &World, ctx: &mut Rltk);
    fn handle(&self, ecs: &mut World, ctx: &mut Rltk);
    fn enter(&self) {
        println!("Entered {} screen", self.name());
    }
    fn exit(&self) {
        println!("Exited {} screen", self.name());
    }
}

#[derive(Clone)]
pub struct MainScreen {}

impl Screen for MainScreen {
    fn name(&self) -> &str { "main" }
    fn render(&self, ecs: &World, ctx: &mut Rltk) {
        let map = ecs.fetch::<Map>();
        let player = ecs.fetch::<Entity>();
        let pos_comp = ecs.read_storage::<Position>();
        let player_pos = pos_comp.get(*player);
        if let Some(player_pos) = player_pos {
            draw_map(&map, player_pos, ctx);
            let positions = ecs.read_storage::<Position>();
            let renderables = ecs.read_storage::<Renderable>();
            for (pos, render) in (&positions, &renderables).join() {
                let (sx, sy) = (*map).map_to_screen(pos.x, pos.y);
                // println!("{},{}", sx, sy);
                if between(sx, 0, 49) && between(sy, 0, 29) {
                    ctx.set(sx, sy, render.fg, render.bg, render.glyph);
                }
            }
        }
    }
    fn handle(&self, ecs: &mut World, ctx: &mut Rltk) {
        match ctx.key {
            None => {}
            Some (key) => match key {
                VirtualKeyCode::Numpad8 => try_move_player(0, -1, ecs),
                VirtualKeyCode::Numpad9 => try_move_player(1, -1, ecs),
                VirtualKeyCode::Numpad6 => try_move_player(1, 0, ecs),
                VirtualKeyCode::Numpad3 => try_move_player(1, 1, ecs),
                VirtualKeyCode::Numpad2 => try_move_player(0, 1, ecs),
                VirtualKeyCode::Numpad1 => try_move_player(-1, 1, ecs),
                VirtualKeyCode::Numpad4 => try_move_player(-1, 0, ecs),
                VirtualKeyCode::Numpad7 => try_move_player(-1, -1, ecs),
                _ => {}
            }
        }
    }
}

impl MainScreen {
    pub fn new() -> Self { MainScreen{} }
}


fn draw_map(map: &Map, pos: &Position, ctx: &mut Rltk) {
    let mut fg: RGB = RGB::named(rltk::BLACK);
    let mut glyph = '\x00';
    
    for x in 0..50 {
        for y in 0..30 {
            let (mx, my) = map.screen_to_map(x, y, pos.x, pos.y);
            let tile = map.get_tile(mx, my);
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
        }
    }
}
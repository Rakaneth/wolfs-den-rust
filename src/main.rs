rltk::add_wasm_support!();
use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode, RandomNumberGenerator};
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;
mod components;
use components::*;
mod map;
use map::*;
mod utils;
mod player;
use player::{try_move_player};

struct State{
    ecs: World,
    rng: RandomNumberGenerator,
}

impl State {
    fn run_systems(&mut self) {
        // let mut m = Mover{};
        // m.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();
        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


fn player_input(gs: &mut State, ctx: &mut Rltk) {
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

fn draw_map(map: &Map, ctx: &mut Rltk) {
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

fn main() {
    let context = Rltk::init_simple8x16(100, 40, "Hello RLTK", "resources");
    let mut gs = State{
        ecs: World::new(),
        rng: RandomNumberGenerator::seeded(0xDEADBEEF),
    };
    gs.ecs.insert(Map::new_random(50, 30, &mut gs.rng));
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Mobile>();
    gs.ecs.register::<Player>();
    gs.ecs
        .create_entity()
        .with(Position{x: 5, y: 5})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::from_u8(191, 121, 101),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();
    rltk::main_loop(context, gs);
}

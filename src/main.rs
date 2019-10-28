rltk::add_wasm_support!();
use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode, RandomNumberGenerator};
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct Mobile {}

#[derive(Component, Debug)]
struct Player {}

struct State{
    ecs: World,
    rng: RandomNumberGenerator,
}

impl State {
    fn run_systems(&mut self) {
        // let mut m = Mover{};
        // m.run_now(&self.ecs);
        // self.ecs.maintain();
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

fn clamp<T: PartialOrd + Copy> (val: T, low: T, high: T) -> T {
    match &val {
        x if x < &low => low,
        x if x > &high => high,
        _ => val
    }
}

fn between<T: PartialOrd + Copy> (val: T, low: T, high: T) -> bool {
    clamp(val, low, high) == val
}

fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = clamp(pos.x + dx, 0, 100);
        pos.y = clamp(pos.y + dy, 0, 40);
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor, Door(bool), NullTile
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width : i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width) as usize + x as usize
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        between(x, 0, self.width - 1) && between(y, 0, self.height - 1)
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        if self.in_bounds(x, y) {
            return self.tiles[self.xy_idx(x, y)];
        } else {
            return TileType::NullTile;
        }
    }

    pub fn can_support_door(&self, x: i32, y: i32) -> bool {
        (self.get_tile(x-1, y) == TileType::Wall && self.get_tile(x+1, y) == TileType::Wall) ||
        (self.get_tile(x, y-1) == TileType::Wall && self.get_tile(x, y+1) == TileType::Wall)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, t: TileType) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = t;
    }

    pub fn new_random(w: i32, h: i32, rng: &mut RandomNumberGenerator) -> Self {
        let mut map = Map{
            tiles: vec![TileType::Floor; (w * h) as usize],
            width: w,
            height: h
        };
        
        for x in 0..w {
            map.set_tile(x, 0, TileType::Wall);
            map.set_tile(x, h - 1, TileType::Wall);
        }

        for y in 0..h {
            map.set_tile(0, y, TileType::Wall);
            map.set_tile(w - 1, y, TileType::Wall);
        }

        for _i in 0..100 {
            let x = rng.roll_dice(1, map.width - 1);
            let y = rng.roll_dice(1, map.height - 1);
            if map.can_support_door(x, y) {
                map.set_tile(x, y, TileType::Door(false));
            } else {
                map.set_tile(x, y, TileType::Wall);
            }
        }
        map
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
        if x > map.width {
            x = 0;
            y += 1;
        }
    }
}

fn main() {
    let context = Rltk::init_simple8x16(100, 40, "Hello RLTK", "resources");
    let mut gs = State{
        ecs: World::new(),
        rng: RandomNumberGenerator::seeded(0xDEADBEEF),
    };
    gs.ecs.insert(Map::new_random(50, 50, &mut gs.rng));
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

rltk::add_wasm_support!();
use rltk::{Rltk, GameState, Console, RGB, RandomNumberGenerator};
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;
mod components;
use components::*;
mod map;
use map::*;
mod utils;
use utils::{between};
mod player;

mod ui;
use ui::*;

pub struct State{
    pub ecs: World,
    pub rng: RandomNumberGenerator,
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
        let player = self.ecs.fetch::<Entity>();
        let pos_comp = self.ecs.read_storage::<Position>();
        let player_pos = pos_comp.get(*player);
        if let Some(player_pos) = player_pos {
            draw_map(&map, player_pos, ctx);
            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();
            for (pos, render) in (&positions, &renderables).join() {
                let (sx, sy) = (*map).map_to_screen(pos.x, pos.y);
                // println!("{},{}", sx, sy);
                if between(sx, 0, 49) && between(sy, 0, 29) {
                    ctx.set(sx, sy, render.fg, render.bg, render.glyph);
                }
            }
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
    gs.ecs.register::<Identity>();
    let player = gs.ecs
        .create_entity()
        .with(Position{x: 5, y: 5})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::from_u8(191, 121, 101),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();
    gs.ecs.insert(player);
    rltk::main_loop(context, gs);
}

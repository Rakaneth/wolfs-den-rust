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
        draw_map(&map, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
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

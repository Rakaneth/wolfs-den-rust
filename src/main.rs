rltk::add_wasm_support!();
use rltk::{Rltk, GameState, Console, RGB, RandomNumberGenerator};
use specs::prelude::*;
use std::collections::HashMap;
#[macro_use]
extern crate specs_derive;
mod components;
use components::*;
mod map;
use map::*;
mod utils;
mod player;
mod factory; 
use factory::*;
mod ui;
use ui::*;

pub struct State{
    pub ecs: World,
    pub cur_screen: Box<dyn Screen>,
    screens: HashMap<String, Box<dyn Screen>>,
}

impl State {
    fn run_systems(&mut self) {
        // let mut m = Mover{};
        // m.run_now(&self.ecs);
        self.ecs.maintain();
    }

    pub fn set_screen(&mut self, name: &str) {
        self.cur_screen.exit();
        let scr = self.screens[name];
        self.cur_screen = scr;

    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();
    }
}

fn main() {
    let context = Rltk::init_simple8x16(100, 40, "Hello RLTK", "resources");
    let mut gs = State{
        ecs: World::new(),
        cur_screen: Box::new(MainScreen{}),
        screens: HashMap::new(),
    };
    gs.ecs.insert(RandomNumberGenerator::seeded(0xDEADBEEF));
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Mobile>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Identity>();
    let player = spawn_player("Player", 5, 5, &mut gs.ecs);
    let map;
    gs.ecs.insert(player);
    {
        let mut rng = gs.ecs.fetch_mut::<RandomNumberGenerator>();
        map = Map::new_random(100, 100, &mut *rng);
    }
    gs.ecs.insert(map);
    rltk::main_loop(context, gs);
}

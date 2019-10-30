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

pub struct ScreenManager {
    pub screens: Vec<Box<dyn Screen>>,
}

impl ScreenManager {
    pub fn new() -> Self {
        ScreenManager {screens: Vec::new()}
    }

    pub fn push_screen(&mut self, scr: Box<dyn Screen>) {
        self.screens.push(scr);
        scr.enter();
    }

    pub fn pop_screen(&mut self) -> Option<Box<dyn Screen>> {
        let to_exit = self.screens.pop();
        if let Some(to_exit) = to_exit {
            to_exit.exit();
        }
        to_exit
    }

    pub fn clear_screens(&mut self) {
        while !self.screens.is_empty() {
            self.pop_screen();
        }
    }

    pub fn cur_screen(&mut self) -> Option<&mut Box<dyn Screen>> {
        if self.screens.is_empty() {
            panic!("Cannot get current screen from empty screen stack!");
        }
        self.screens.last_mut()
    }
}



pub struct State{
    pub ecs: World,
    pub rng: RandomNumberGenerator,
    pub sm: ScreenManager,
}

impl State {
    fn run_systems(&mut self) {
        // let mut m = Mover{};
        // m.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn handle(&mut self, ctx: &mut Rltk) {
        {
            let last = self.sm.cur_screen().unwrap();
        }
        last.handle(self, ctx);
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
        rng: RandomNumberGenerator::seeded(0xDEADBEEF),
        sm: ScreenManager::new(),
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

rltk::add_wasm_support!();
use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
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
    ecs: World
}

struct Mover{}

impl<'a> System<'a> for Mover {
    type SystemData = (ReadStorage<'a, Mobile>, WriteStorage<'a, Position>);
    
    fn run(&mut self, (mob, mut pos): Self::SystemData) {
        for (_mob, pos) in (&mob, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {pos.x = 99; }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut m = Mover{};
        m.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn clamp<T: PartialOrd> (val: T, low: T, high: T) -> T {
    match &val {
        x if x < &low => low,
        x if x > &high => high,
        _ => val
    }
}

fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = clamp(pos.x + dx, 0, 100);
        pos.y = clamp(pos.y + dy, 0, 40);
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

fn main() {
    let context = Rltk::init_simple8x16(100, 40, "Hello RLTK", "resources");
    let mut gs = State{
        ecs: World::new()
    };
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
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position{ x: i * 5, y: 10})
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Mobile{})
            .build();
    }

    rltk::main_loop(context, gs);
}

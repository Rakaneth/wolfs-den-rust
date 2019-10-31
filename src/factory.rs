use specs::prelude::*;
use super::components::*;
use rltk::{RGB};

pub fn spawn_player(name: &str, x: i32, y: i32, ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Position{x: x, y: y})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::from_u8(191, 121, 101),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Identity{
            name: String::from(name),
            id: String::from("player"),
        })
        .build()
}
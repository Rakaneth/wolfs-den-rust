use specs::prelude::*;
use rltk::RGB;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Mobile {}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Identity {
    pub name: String,
    pub id: String,
}
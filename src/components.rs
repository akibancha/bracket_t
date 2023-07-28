use bracket_lib::prelude::{FontCharType, RGB};
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Pos{
    pub x:i32,
    pub y: i32
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB
}


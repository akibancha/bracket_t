#![allow(unused)]
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;


pub struct State {
   pub  ecs: World
}
 impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
 }

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {

        player_input(self, ctx);

        ctx.cls();

        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        let posis = self.ecs.read_storage::<Pos>();
        let renders = self.ecs.read_storage::<Renderable>();

        draw_map(&map, ctx);

        for (pos, render) in (&posis, &renders).join() {
            ctx.set(pos.x, pos.y, render.fg,render.bg, render.glyph)
        }
    }
}

fn main() -> bracket_lib::prelude::BError {


    let mut gs = State {
        ecs: World::new()

    };

    gs.ecs.insert(new_map());
    gs.ecs.register::<Pos>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Pos{x:40, y:25})
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK)
        })
        .with(Player{})
        .build();

    let context = BTermBuilder::simple80x50()
        .with_title("Huhu World!")
        .build()?;

    main_loop(context, gs)
}

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
pub use rect::Rect;
mod visibility_system;
pub use visibility_system::VisibilitySystem;

pub struct State {
   pub  ecs: World
}
 impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
 }

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {

        player_input(self, ctx);

        ctx.cls();

        self.run_systems();

        let posis = self.ecs.read_storage::<Position>();
        let renders = self.ecs.read_storage::<Renderable>();

        let map = self.ecs.fetch::<Map>();
        draw_map(&self.ecs, ctx);

        for (pos, render) in (&posis, &renders).join() {
            ctx.set(pos.x, pos.y, render.fg,render.bg, render.glyph)
        }
    }
}

fn main() -> bracket_lib::prelude::BError {

    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    let map: Map = Map::new_map_rooms_and_corridors();

    let (player_x, player_y): (i32, i32) = map.rooms[0].center();

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        gs.ecs.create_entity()
            .with(Position{x, y})
            .with(Renderable{
                glyph: to_cp437('r'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
            .build();
    }

    gs.ecs
        .create_entity()
        .with(Position{x:player_x, y:player_y})
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK)
        })
        .with(Player{})
        .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
        .build();

    let context = BTermBuilder::simple80x50()
        .with_title("Huhu World!")
        .build()?;

    gs.ecs.insert(map);

    main_loop(context, gs)
}

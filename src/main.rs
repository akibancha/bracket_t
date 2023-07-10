#![allow(unused)]
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;



#[derive(Component)]
struct Pos{
    x:i32,
    y: i32
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB
}

struct State {
    ecs: World
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let posis = self.ecs.read_storage::<Pos>();
        let renders = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&posis, &renders).join() {
            ctx.set(pos.x, pos.y, render.fg,render.bg, render.glyph)
        }
    }
}


fn main() -> bracket_lib::prelude::BError {


    let mut gs = State {
        ecs: World::new()

    };

    gs.ecs.register::<Pos>();
    gs.ecs.register::<Renderable>();

    gs.ecs
        .create_entity()
        .with(Pos{x:40, y:25})
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK)
        })
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Pos { x: i * 7, y: 20})
            .with(Renderable {
                glyph: to_cp437('>'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK)
            })
        .build();
    }

    let context = BTermBuilder::simple80x50()
        .with_title("Huhu World!")
        .build()?;

    main_loop(context, gs)
}

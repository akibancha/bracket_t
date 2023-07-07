use bracket_lib::prelude::*;

struct State { }

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let out = "Huhu world!";
        ctx.cls();
        ctx.print(40 - out.len() as i32 / 2 as i32, 50 / 2 as i32, out);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello world")
        .build()?;

    let gs: State = State { };


    main_loop(context, gs)
}

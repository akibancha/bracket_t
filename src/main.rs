#![allow(unused)]
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor
}

pub fn xy_idx(x: i32, y:i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    for x in 0..80{
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        match tile {

            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('.'))
            }
 
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), to_cp437('#'))
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

#[derive(Component)]
struct Player {}

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

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Pos>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(79, max(0, pos.y + delta_y));        }
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    // Player movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
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

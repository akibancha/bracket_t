use bracket_lib::prelude::{VirtualKeyCode, BTerm};
use specs::prelude::*;
use crate::Viewshed;

use super::{Position, Player, TileType, Map, State};
use std::cmp::{min, max};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(79, max(0, pos.y + delta_y));        }
            viewshed.dirty = true;
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    // Player movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            
            // move west
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H 
                => try_move_player(-1, 0, &mut gs.ecs),

            // move east
            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L
                => try_move_player(1, 0, &mut gs.ecs),

            // move north
            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K 
                => try_move_player(0, -1, &mut gs.ecs),

            // move south
            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J 
                => try_move_player(0, 1, &mut gs.ecs),

            // no valid input
            _ => {}
        }
    }
}

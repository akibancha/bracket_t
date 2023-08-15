use::bracket_lib::prelude::{RGB, BTerm, RandomNumberGenerator, to_cp437, Algorithm2D, BaseMap, Point};
use std::cmp::{min, max};
use super::Rect;


#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : i32,
    pub height : i32
}


impl Map {
 
    // Translates x and y in to an index in the one-dimensional map vector.
    pub fn xy_idx(&self, x: i32, y:i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

     // Applies a room to the map.
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        } 
    }

    // Applies a horizontal corridor to the map.
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32){
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    // Applies a vertical corridor to the map.
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32){
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    // Creates a map with rectangulat rooms and corridors that connect them.
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map{
            tiles : vec![TileType::Wall; 80*50],
            rooms : Vec::new(),
            width : 80,
            height : 50
            };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {

            let w: i32 = rng.range(MIN_SIZE, MAX_SIZE);
            let h: i32 = rng.range(MIN_SIZE, MAX_SIZE);

            let x: i32 = rng.roll_dice(1, 80 - w -1) -1; 
            let y: i32 = rng.roll_dice(1, 50 - h -1) -1; 

            let new_room: Rect = Rect::new(x, y, w, h);
            let mut ok: bool = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y): (i32, i32) = new_room.center();
                    let (prev_x, prev_y): (i32, i32) = map.rooms[map.rooms.len()-1].center();
                    if rng.range(0,2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    }
                    
                }   
                map.rooms.push(new_room);
            }
        }
        map
    }

} // impl end

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
    self.tiles[idx as usize] == TileType::Wall
    } 
}


// Creates a map with soild boundaries and 400 randomly placed walls.
//pub fn new_map_test() -> Vec<TileType> {
//    let mut map = vec![TileType::Floor; 80*50];
//
//    for x in 0..80{
//        map[xy_idx(x, 0)] = TileType::Wall;
//        map[xy_idx(x, 49)] = TileType::Wall;
//    }
//
//    for y in 0..50 {
//        map[xy_idx(0, y)] = TileType::Wall;
//        map[xy_idx(79, y)] = TileType::Wall;
//    }
//
//    let mut rng = RandomNumberGenerator::new();
//
//    for _i in 0..400 {
//        let x = rng.roll_dice(1, 79);
//        let y = rng.roll_dice(1, 49);
//        let idx = xy_idx(x, y);
//        if idx != xy_idx(40, 25) {
//            map[idx] = TileType::Wall;
//        }
//    }
//
//    map
//}
//

// Draws the map to a BTerm.
pub fn draw_map(map: &Map, ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.tiles.iter() {
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



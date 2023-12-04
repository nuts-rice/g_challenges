use super::{Dungeon, Room, TileType};
use std::cmp::{max, min};

pub fn apply_room_to_dungeon(dungeon: &mut Dungeon, room: Room) {
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            let idx = (dungeon.width * y + x) as usize;
            dungeon.tiles[idx] = TileType::Floor;
        }
    }
}

pub fn apply_horizontal_tunnel(dungeon: &mut Dungeon, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = (dungeon.width * y + x) as usize;

        if idx > 0 && idx < dungeon.width as usize * dungeon.height as usize {
            dungeon.tiles[idx] = TileType::Floor;
        }
    }
}

pub fn apply_vertical_tunnel(dungeon: &mut Dungeon, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = (dungeon.width * y + x) as usize;
        if idx > 0 && idx < dungeon.width as usize * dungeon.height as usize {
            dungeon.tiles[idx] = TileType::Floor;
        }
    }
}

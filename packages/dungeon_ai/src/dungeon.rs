use crate::*;
use comfy::epaint::Pos2;
use ldtk_rust::Project;

pub const DUNGEONWIDTH: f32 = 40.;
pub const DUNGEONHEIGHT: f32 = 21.;
pub const DUNGEONSIZE: f32 = DUNGEONWIDTH * DUNGEONHEIGHT;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

#[derive(Default, Debug, Clone)]
pub struct Dungeon {
    pub tiles: Vec<TileType>,
    pub rects: Vec<Rect>,
    pub rooms: Vec<Rect>,
    pub walls: Vec<bool>,
    pub width: f32,
    pub height: f32,
    pub revealed_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub seed: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct Room {
    top_left: Pos2,
    bottom_right: Pos2,
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    top_left: Pos2,
    bottom_right: Pos2,
}
impl Rect {
    pub fn new(top_left: Pos2, bottom_right: Pos2) -> Rect {
        Rect {
            top_left,
            bottom_right,
        }
    }
    pub fn intersects(&self, other: &Rect) -> bool {
        self.top_left.x <= other.bottom_right.x
            && self.bottom_right.x >= other.top_left.x
            && self.top_left.y <= other.bottom_right.y
            && self.bottom_right.y >= other.top_left.y
    }
    pub fn center(&self) -> Pos2 {
        Pos2 {
            x: (self.top_left.x + self.bottom_right.x) / 2.,
            y: (self.top_left.y + self.bottom_right.y) / 2.,
        }
    }
}

pub fn apply_room_to_map(room: &Rect, map: &mut Dungeon) {
    for y in room.top_left.y as i32..=room.bottom_right.y as i32 {
        for x in room.top_left.x as i32..=room.bottom_right.x as i32 {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }
}

pub fn spawn_room(map: &mut Dungeon, room: &Rect, _spawn_list: &mut Vec<(usize, String)>) {
    let mut canidates: Vec<usize> = Vec::new();
    {
        for y in room.top_left.y as i32..=room.bottom_right.y as i32 {
            for x in room.top_left.x as i32..=room.bottom_right.x as i32 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == TileType::Floor {
                    canidates.push(idx);
                }
            }
        }
    }
    spawn_region();
}

pub fn spawn_region() {
    unimplemented!()
}
impl Dungeon {
    pub fn new() -> Self {
        Dungeon {
            tiles: vec![TileType::Wall; (DUNGEONWIDTH * DUNGEONHEIGHT) as usize],
            rects: Vec::new(),
            rooms: Vec::new(),
            walls: vec![false; (DUNGEONWIDTH * DUNGEONHEIGHT) as usize],
            width: DUNGEONWIDTH,
            height: DUNGEONHEIGHT,
            revealed_tiles: vec![false; (DUNGEONWIDTH * DUNGEONHEIGHT) as usize],
            blocked: vec![false; (DUNGEONWIDTH * DUNGEONHEIGHT) as usize],
            depth: 0,
            seed: 0,
        }
    }
    pub fn generate_map(&mut self, _depth: i32, _seed: u64) {
        self.build_bsp();
    }
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn load_map() {
        let _ldtk = Project::load_project("assets/level.ldtk");
    }

    fn build_bsp(&mut self) {
        self.rects.clear();
        self.rects.push(Rect::new(
            Pos2 { x: 2., y: 2. },
            Pos2 {
                x: self.width - 5.,
                y: self.height - 5.,
            },
        ));
        let first_room = self.rects[0];
        self.add_subrects(first_room);
        let mut n_rooms = 0;
        while n_rooms < 40 {
            let rect = self.get_random_rect();
            let canidate = self.get_random_sub_rect(rect);
            info!(
                "canidate room has corners of : {:?} and {:?}",
                canidate.top_left, canidate.bottom_right
            );
            if self.is_bsp_possible(canidate) {
                apply_room_to_map(&canidate, self);
                self.rooms.push(canidate);
                self.add_subrects(rect);
            }
            n_rooms += 1;
        }
        self.rooms
            .sort_by(|a, b| (a.top_left.x as i32).cmp(&(b.top_left.x as i32)));
        for i in 0..self.rooms.len() - 1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i + 1];
            let start_x = room.top_left.x
                + (random_range(1., f32::abs(room.top_left.x - room.bottom_right.x)) - 1.);
            let start_y = room.top_left.y
                + (random_range(1., f32::abs(room.top_left.y - room.bottom_right.y)) - 1.);
            let end_x = next_room.top_left.x
                + (random_range(
                    1.,
                    f32::abs(next_room.top_left.x - next_room.bottom_right.x),
                ) - 1.);
            let end_y = next_room.top_left.y
                + (random_range(
                    1.,
                    f32::abs(next_room.top_left.y - next_room.bottom_right.y),
                ) - 1.);
            self.draw_corridor(start_x, start_y, end_x, end_y);
        }
    }
    fn is_bsp_possible(&self, _canidate: Rect) -> bool {
        let mut expanded = _canidate;
        expanded.top_left.x -= 2.;
        expanded.top_left.y -= 2.;
        expanded.bottom_right.x += 2.;
        expanded.bottom_right.y += 2.;
        let mut can_build = true;
        for y in expanded.top_left.y as i32..=expanded.bottom_right.y as i32 {
            for x in expanded.top_left.x as i32..=expanded.bottom_right.x as i32 {
                if x > (self.width - 2.) as i32 {
                    can_build = false;
                }
                if y > (self.height - 2.) as i32 {
                    can_build = false;
                }
                if x < 1 {
                    can_build = false;
                }
                if y < 1 {
                    can_build = false;
                }
                if can_build {
                    let idx = self.xy_idx(x, y);
                    if self.tiles[idx] != TileType::Wall {
                        can_build = false;
                    }
                }
            }
        }
        can_build
    }

    fn draw_corridor(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32) {
        let mut x = _x1;
        let mut y = _y1;
        while x != _x2 || y != _y2 {
            if x < _x2 {
                x += 1.;
            } else if x > _x2 {
                x -= 1.;
            } else if y < _y2 {
                y += 1.;
            } else if y > _y2 {
                y -= 1.;
            }
            let idx = self.xy_idx(x as i32, y as i32);
            self.tiles[idx] = TileType::Floor;
        }
    }

    fn get_random_sub_rect(&self, _canidate: Rect) -> Rect {
        let mut result = _canidate;
        let rect_width = i32::abs(_canidate.top_left.x as i32 - _canidate.bottom_right.x as i32);
        let rect_height = i32::abs(_canidate.top_left.y as i32 - _canidate.bottom_right.y as i32);
        let w = i32::max(2, random_i32(1, i32::min(rect_width, 10)) - 1) + 1;
        let h = i32::max(2, random_i32(1, i32::min(rect_height, 10)) - 1) + 1;
        result.top_left.x += random_range(1., 6.) - 1.;
        result.top_left.y += random_range(1., 6.) - 1.;
        result.bottom_right.x = result.top_left.x + w as f32;
        result.bottom_right.y = result.top_left.y + h as f32;
        result
    }
    fn get_random_rect(&self) -> Rect {
        if self.rects.len() == 1 {
            return self.rects[0];
        }
        let idx = random_usize(1, self.rects.len() - 1);
        self.rects[idx]
    }
    fn add_subrects(&mut self, _rect: Rect) {
        let width = i32::abs(_rect.top_left.x as i32 - _rect.bottom_right.x as i32);
        let height = i32::abs(_rect.top_left.y as i32 - _rect.bottom_right.y as i32);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);
        self.rects.push(Rect::new(
            _rect.top_left,
            Pos2 {
                x: half_width as f32,
                y: half_height as f32,
            },
        ));
        self.rects.push(Rect::new(
            Pos2 {
                x: _rect.top_left.x,
                y: _rect.top_left.y + half_height as f32,
            },
            Pos2 {
                x: half_width as f32,
                y: half_height as f32,
            },
        ));
        self.rects.push(Rect::new(
            Pos2 {
                x: _rect.top_left.x + half_width as f32,
                y: _rect.top_left.y,
            },
            Pos2 {
                x: half_width as f32,
                y: half_height as f32,
            },
        ));
        self.rects.push(Rect::new(
            Pos2 {
                x: _rect.top_left.x + half_width as f32,
                y: _rect.top_left.y + half_height as f32,
            },
            Pos2 {
                x: half_width as f32,
                y: half_height as f32,
            },
        ));
    }
}

// impl fmt::Display for Dungeon {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut s = String::new();
//         for y in 0..self.height {
//
//             for x in 0..self.width {
//
//                 let idx = self.xy_idx(x, y);
//                 match self.tiles[idx] {
//                     TileType::Floor => s.push('.'),
//                     TileType::Wall => s.push('#'),
//                     TileType::DownStairs => s.push('>'),
//                 }
//             }
//             s.push('\n');
//         }
//         write!(f, "{}", s)
//     }
// }

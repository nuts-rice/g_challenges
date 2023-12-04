use std::borrow::BorrowMut;

use super::{apply_horizontal_tunnel, apply_vertical_tunnel};
use crate::*;
pub use image::{DynamicImage, GenericImage, RgbaImage};
pub use rand::{Rng, SeedableRng};
pub use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
pub use wfc::{Context, ForbidInterface, ForbidPattern, PatternId, RunBorrow, Size, Wave, Wrap};
pub use wfc_image::ImagePatterns;
pub use wfc_image::*;
pub const DUNGEONWIDTH: usize = 30;
pub const DUNGEONHEIGHT: usize = 30;
pub const DUNGEONCOUNT: usize = DUNGEONHEIGHT * DUNGEONWIDTH;
pub const MAX_ROOMS: i32 = 12;
const BASE_COLOR: Color = WHITE;
pub type DungeonAscii = Vec<Vec<char>>;

#[derive(PartialEq)]
enum Tileset {
    Numbers,
    Rooms,
    Knots,
    Castle,
}
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

static TILESET: AtomicRefCell<Tileset> = AtomicRefCell::new(Tileset::Rooms);
static SHOWHISTORY: AtomicRefCell<bool> = AtomicRefCell::new(true);
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Room {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}
impl Room {
pub fn new(x:i32, y: i32, w:i32, h:i32) -> Room {
        Room{x1:x, y1:y, x2:x+w, y2:y+h}
    }
 pub fn intersect(&self, other:&Room) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }

}

pub struct GameState {
    // pub pixels: Grid<Color>,
    // pub image: DynamicImage,
    pub current_dungeon: Dungeon,
    pub dungeon_gen_idx: usize,
    pub dungeon_gen_timer: f32,
    pub dungeon_gen_history: Vec<Dungeon>,
    // pub handle: TextureHandle,
}
impl GameState {
    pub fn new(_c: &mut EngineContext) -> Self {
        let ratio = screen_width() / screen_height();
        println!("ratio: {}", ratio);
        let width = 48;
        let _height = (width as f32 / ratio) as i32;
        let current_dungeon = Dungeon::new(1, 123456);
        let dungeon_gen_idx: usize = 0;
        let dungeon_gen_timer: f32 = 0.0;
        let dungeon_gen_history: Vec<Dungeon> = Vec::new();
        GameState {
            // pixels,
            // image,
            current_dungeon,
            dungeon_gen_idx,
            dungeon_gen_timer,
            dungeon_gen_history,
            // handle,
        }
    
    }
    pub fn draw_walls(&mut self) {
        for (idx, tile) in self.current_dungeon.tiles.iter_mut().enumerate() {
            self.current_dungeon.walls[idx] = *tile == TileType::Wall;
        }
    }
    pub fn generate_map(&mut self, new_depth: i32, seed: u64, _c: &mut EngineContext) {
        // self.dungeon_gen_idx = 0;
        // self.dungeon_gen_timer = 0.0;
        // self.dungeon_gen_history.clear();
        // clear_dungeon(self);
        let mut builder = SimpleBuilder::new(new_depth, seed);
        builder.build_dungeon();
        let _dungeon = builder.get_dungeon();
        self.dungeon_gen_history = builder.get_snapshot_history();
        self::simple_draw(self, _c)
    }
}

pub fn draw_dungeon(_dungeon: &Dungeon, _c: &mut EngineContext) {
    println!("Drawing dungeon...");
    let mut _y = 0;
    let mut _x = 0;
    for (idx, tile) in _dungeon.tiles.iter().enumerate() {
        if _dungeon.revealed[idx] {
            match tile {
                TileType::Floor => {
                    let floor_color = Color::new(0.0, 0.62, 0.62, 1.);
                    draw_rect(vec2(_x as f32, _y as f32), splat(1.0), floor_color, 0);
                    // println!("Floor");
                }
                TileType::Wall => {
                    let wall_color = Color::new(0.0, 1., 0., 1.);
                    draw_rect(vec2(_x as f32, _y as f32), splat(1.0), wall_color, 0);

                    // _state.pixels.data[idx ]    = Color::new(0.0, 1., 0., 0.) ;
                    // println!("Wall");
                }
                TileType::DownStairs => {
                    let _down_stairs_color = Color::new(0.0, 1., 1., 1.);
                    println!("down stairs");
                    // _state.pixels.data[idx ]  = Color::new(0.0, 1., 1., 0.) ;
                }
            }
            _x += 1;
            if _x > DUNGEONWIDTH as i32 - 1 {
                _x = 0;
                _y += 1;
            }
        }
    }
}
pub fn game_update(state: &mut GameState, _c: &mut EngineContext) {
    // _c.texture_creator
    //     .borrow_mut()
    //     .update_texture(&state.image, state.handle);
    clear_background(GRAY);

    // for (x, y, color) in state.pixels.iter() {
    //     state
    //         .image
    //         .put_pixel(x as u32, y as u32, color.to_image_rgba());
    // }

    let _viewport = main_camera_mut().world_viewport();
    main_camera_mut().center = Vec2::from([_viewport.x / 2., _viewport.y / 2.]);
    if is_key_pressed(KeyCode::V) {
        println!("toggling visibility ");
        let mut show_history = SHOWHISTORY.borrow_mut();
        *show_history = !*show_history;
    }
    if is_key_pressed(KeyCode::R) {
        println!("Rerolling dungeon...");
        self::clear_dungeon(state);
        let seed = rand::thread_rng().gen_range(0..=u64::MAX);
        state.dungeon_gen_idx += 1;
        state
            .dungeon_gen_history
            .push(state.current_dungeon.clone());
        state.generate_map(1, seed, _c);
        self::simple_draw(state, _c);
        // self::draw_dungeon(&state.current_dungeon.clone(), _c);
    }
    if is_key_pressed(KeyCode::A) {
        println!("Printing ACSII map...");
        for row in state.current_dungeon.ascii.iter() {
            println!("{:?}", row);
        }
    }
}

pub fn clear_dungeon(state: &mut GameState) {
    println!("Clearing dungeon");
    for tile in state.current_dungeon.tiles.iter_mut() {
        *tile = TileType::Floor;
    }
}

pub fn simple_draw(state: &mut GameState, _c: &mut EngineContext) {
    println!("SIMPLE DRAW...");
    let mut y = 0.;
    let mut x = 0.;

    for (idx, tile) in state.current_dungeon.tiles.iter().enumerate() {
        // if state.current_dungeon.revealed[idx] {
        match tile {
            TileType::Floor => {
                draw_sprite(texture_id("floor"), vec2(x, y), WHITE, 0, splat(2.0));
                state.current_dungeon.ascii[y as usize][x as usize] = '.';
            }
            TileType::Wall => {
                draw_sprite(texture_id("wall"), vec2(x, y), WHITE, 0, splat(5.0));

                state.current_dungeon.ascii[y as usize][x as usize] = '+';
            }
            TileType::DownStairs => {
                draw_rect(vec2(x + 1., y + 1.), splat(0.4), BROWN.alpha(0.5), 0);
                state.current_dungeon.ascii[y as usize][x as usize] = '>';
            }
        }
        x += 1.;
        if x > DUNGEONWIDTH as f32 - 1. {
            x = 0.;
            y += 1.;
        }
    }
// }
 }

pub fn draw_room_boundary(dungeon: &mut Dungeon, room: &Room) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            let idx = (y as usize * dungeon.width as usize) + x as usize;
            //if idx > 0 && idx < ((dungeon.width * dungeon.height) - 1) as usize {
            if x == room.x1  || x == room.x2 || y == room.y1 || y == room.y2 {
                dungeon.tiles[idx] = TileType::Wall;
            } else {
                dungeon.tiles[idx] = TileType::Floor;
                // } else {
                //     self.dungeon.tiles[idx] = TileType::Floor;
            }
        }
    }
}
#[inline]
pub fn is_possible(dungeon: Dungeon, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) -> bool {
    x1 -= 2;
    x2 += 2;
    y1 -= 2;
    y2 += 2;
    for y in y1..y2 {
        for x in x1..x2 {
            if x < 1 || x > dungeon.width - 2 || y < 1 || y > dungeon.height - 2 {
                return false;
            }
        }
    }
    true
}

pub trait DungeonBuilder {
    fn build_dungeon(&mut self);
    fn spawn_etities(&mut self, hecs: &mut World);
    fn get_dungeon(&self) -> Dungeon;
    fn get_start_position(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Dungeon>;
    fn take_snapshot(&mut self);
}
pub struct BSPBuilder {
    dungeon: Dungeon,
    start_position: Position,
    depth: i32,
    rooms: Vec<Room>,
    dungeon_gen_history: Vec<Dungeon>,
}

pub struct MidpointBuilder {
    dungeon: Dungeon,
    start_position: Position,
    depth: i32,
    rooms: Vec<Room>,
    dungeon_gen_history: Vec<Dungeon>,
}

pub struct SimpleBuilder {
    dungeon: Dungeon,
    start_position: Position,
    depth: i32,
    rooms: Vec<Room>,
    rects: Vec<Room>,
    dungeon_gen_history: Vec<Dungeon>,
}
impl DungeonBuilder for SimpleBuilder {
    fn get_dungeon(&self) -> Dungeon {
        self.dungeon.clone()
    }
    fn get_start_position(&self) -> Position {
        self.start_position
    }
    fn get_snapshot_history(&self) -> Vec<Dungeon> {
        self.dungeon_gen_history.clone()
    }
    fn build_dungeon(&mut self) {
        // self.rooms.clear();

                self.simple_build();
                println!(
            "using simple build for {} by {} dungeon with seed value of : {}",
            self.dungeon.height, self.dungeon.width, self.dungeon.seed
        );

    }
    fn take_snapshot(&mut self) {
        println!("Taking snapshot...");
        let show_history = SHOWHISTORY.borrow_mut();
        if *show_history {
            let mut snapshot = self.dungeon.clone();
            for reveal in snapshot.revealed.iter_mut() {
                *reveal = true;
            }
            self.dungeon_gen_history.push(snapshot);
        }
    }
    fn spawn_etities(&mut self, _hecs: &mut World) {
        unimplemented!()
    }
}
impl SimpleBuilder {
    pub fn new(new_depth: i32, seed: u64) -> SimpleBuilder {
        SimpleBuilder {
            dungeon: Dungeon::new(new_depth, seed),
            start_position: Position { x: 0, y: 0 },
            depth: new_depth,
            rects: Vec::new(),
            rooms: Vec::new(),
            dungeon_gen_history: Vec::new(),
        }
    }
    fn simple_build(&mut self) {
        self.rects.clear();
        self.rects.push(Room::new(2, 2, self.dungeon.width - 5, self.dungeon.height - 5));
        let first_room = self.rects[0];
        let width = i32::abs(first_room.x1 - first_room.x2);
        let height = i32::abs(first_room.y1 - first_room.y2);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);
        self.rects.push(Room::new(
            first_room.x1 + half_width,
            first_room.y1,
            half_width,
            half_height,
        ));
        self.rects.push(Room::new(
            first_room.x1,
            first_room.y1 + half_height,
            half_width,
            half_height,
        ));
        self.rects.push(Room::new(
            first_room.x1 + half_width,
            first_room.y1 + half_height,
            half_width,
            half_height,
        ));
        self.rects.push(Room::new(
            first_room.x1 + half_width,
            first_room.y1 + half_height,
            half_width,
            half_height,
        ));
        let mut rng = ChaCha20Rng::seed_from_u64(self.dungeon.seed);
        for _i in 0..MAX_ROOMS {
            let w = rng.gen_range(6..10);
            let h = rng.gen_range(6..10);
            let x = rng.gen_range(1..self.dungeon.width - w - 1) - 1;
            let y = rng.gen_range(1..self.dungeon.height - h - 1) - 1;
            let candidate = Room::new(x, y, w, h);
            let mut ok = true;
            for other_room in self.rooms.iter() {
                if candidate.intersect(other_room) {
                    ok = false;
                }
            }
            if ok {
                draw_room_boundary(&mut self.dungeon, &candidate);
                self.take_snapshot();
                if !self.rooms.is_empty() {
                let (new_x, new_y) = candidate.center();
                let (prev_x, prev_y) = self.rooms[self.rooms.len()-1].center();
                if rng.gen_range(0..2) == 1 {
                    println!("drawing horizontal tunnel");
                    apply_horizontal_tunnel(&mut self.dungeon, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut self.dungeon, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut self.dungeon, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut self.dungeon, prev_x, new_x, new_y);
                }
            }
            self.rooms.push(candidate);
            println!(
                        "candidate room has boundaries of {},{} to {},{}",
                        candidate.x1, candidate.y1, candidate.x2, candidate.y2
                    );

            self.take_snapshot();
        }
        }
        let start_position = self.rooms[0].center();
        self.start_position = Position {
            x: start_position.0,
            y: start_position.1,
        };
        println!("Starting in position: {:?}", self.start_position);
    }
}
impl DungeonBuilder for MidpointBuilder {
    fn get_dungeon(&self) -> Dungeon {
        self.dungeon.clone()
    }
    fn get_start_position(&self) -> Position {
        self.start_position
    }
    fn get_snapshot_history(&self) -> Vec<Dungeon> {
        self.dungeon_gen_history.clone()
    }
    fn build_dungeon(&mut self) {
        println!(
            "using mispoint displacement for {} by {} dungeon with seed value of : {}",
            self.dungeon.height, self.dungeon.width, self.dungeon.seed
        );
        self.rooms.clear();

        self.displace();
    }
    fn take_snapshot(&mut self) {
        println!("Taking snapshot...");
        let show_history = SHOWHISTORY.borrow_mut();
        if *show_history {
            let mut snapshot = self.dungeon.clone();
            for reveal in snapshot.revealed.iter_mut() {
                *reveal = true;
            }
            self.dungeon_gen_history.push(snapshot);
        }
    }
    fn spawn_etities(&mut self, _hecs: &mut World) {
        unimplemented!()
    }
}

impl MidpointBuilder {
    pub fn new(new_depth: i32, seed: u64) -> MidpointBuilder {
        MidpointBuilder {
            dungeon: Dungeon::new(new_depth, seed),
            start_position: Position { x: 0, y: 0 },
            depth: new_depth,
            rooms: Vec::new(),
            dungeon_gen_history: Vec::new(),
        }
    }
    fn calculate_square_average(
        top_left: f32,
        top_right: f32,
        bottom_left: f32,
        bottom_right: f32,
    ) -> f32 {
        (top_left + top_right + bottom_left + bottom_right) / 4.0
    }

    fn displace(&mut self) {
        let mut rng = ChaCha20Rng::seed_from_u64(self.dungeon.seed);

        for _y in 1..self.dungeon.height - 1 {
            for _x in 1..self.dungeon.width - 1 {
                let rand_val = rng.gen_range(0.0..=1.) as f32;
                // let square_avg = Self::calculate_square_average(
                //     *top_left,
                //     *top_right,
                //     *bottom_left,
                //     *bottom_right,
                // );
                let w = rng.gen_range(6..10);
                let h = rng.gen_range(6..10);
                let x = rng.gen_range(1..self.dungeon.width - w - 1) - 1;
                let y = rng.gen_range(1..self.dungeon.height - h - 1) - 1;
                let square_avg = 0.13;
                let displace = square_avg + rand_val;
                let t = square_avg + displace;
                let x1 = x - t as i32;
                let y1 = y - t as i32;
                let x2 = (x + t as i32) + 2;
                let y2 = (y + t as i32) + 4;
                let mut ok = true;
                let candidate = Room::new(x1, y1, x2, y2);
                for other_room in self.rooms.iter() {
                    if candidate.intersect(other_room) {
                        ok = false;
                    }
                }
                if ok && self.is_possible(x1, x2, y1, y2) {
                    draw_room_boundary(&mut self.dungeon, &candidate);
                    self.take_snapshot();
                    println!(
                        "candidate room has boundaries of {},{} to {},{}",
                        x1, y1, x2, y2
                    );
                    if !self.rooms.is_empty() {
                        let (new_x, new_y) = candidate.center();
                        let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
                        if rng.gen_range(0..2) == 1 {
                            println!("drawing horizontal tunnel");
                            apply_horizontal_tunnel(&mut self.dungeon, prev_x, new_x, prev_y);
                            apply_vertical_tunnel(&mut self.dungeon, prev_y, new_y, new_x);
                        } else {
                            apply_vertical_tunnel(&mut self.dungeon, prev_y, new_y, prev_x);
                            apply_horizontal_tunnel(&mut self.dungeon, prev_x, new_x, new_y);
                        }
                    }
                    self.rooms.push(candidate);
                    self.take_snapshot();
                }
            }
        }
        // let stair_position = Self::center(self.rooms.data[self.rooms.data.len() - 1]);

        // let stairs_idx = (self.dungeon.width * stair_position.0 + stair_position.1) as usize;
        // self.dungeon.tiles[stairs_idx] = TileType::DownStairs;
       let start_position = self.rooms[0].center();
        self.start_position = Position {
            x: start_position.0,
            y: start_position.1,
        };
        println!("Starting in position: {:?}", self.start_position);
    }
    fn room_check(_candidate: Room) -> bool {
        unimplemented!()
    }
    fn is_possible(&self, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) -> bool {
        x1 -= 2;
        x2 += 2;
        y1 -= 2;
        y2 += 2;
        for y in y1..y2 {
            for x in x1..x2 {
                if x < 1 || x > self.dungeon.width - 2 || y < 1 || y > self.dungeon.height - 2 {
                    return false;
                }
            }
        }
        true
    }
}

impl DungeonBuilder for BSPBuilder {
    fn get_dungeon(&self) -> Dungeon {
        self.dungeon.clone()
    }
    fn get_start_position(&self) -> Position {
        self.start_position
    }
    fn get_snapshot_history(&self) -> Vec<Dungeon> {
        self.dungeon_gen_history.clone()
    }
    fn build_dungeon(&mut self) {
        println!(
            "using bsp for {} by {} dungeon with seed value of : {}",
            self.dungeon.height, self.dungeon.width, self.dungeon.seed
        );
        BSPBuilder::build();
    }

    fn take_snapshot(&mut self) {
        println!("Taking snapshot...");
        let show_history = SHOWHISTORY.borrow_mut();
        if *show_history {
            let mut snapshot = self.dungeon.clone();
            for reveal in snapshot.revealed.iter_mut() {
                *reveal = true;
            }
            self.dungeon_gen_history.push(snapshot);
        }
    }
    fn spawn_etities(&mut self, _hecs: &mut World) {
        unimplemented!()
    }
}

impl BSPBuilder {
    fn new() {
        unimplemented!()
    }
    fn build() {
        unimplemented!()
    }
}
#[derive(Clone, Debug)]
pub struct Dungeon {
    pub walls: Vec<bool>,
    pub width: i32,
    pub height: i32,
    pub size: i32,
    pub depth: i32,
    pub bloodstains: HashSet<usize>,
    pub revealed: Vec<bool>,
    pub tiles: Vec<TileType>,
    pub ascii: DungeonAscii,
    pub seed: u64,
}

impl Dungeon {
    fn new(_depth: i32, seed: u64) -> Self {
        Dungeon {
            // input_image: DynamicImage::new_rgba8(0, 0),
            tiles: vec![TileType::Wall; DUNGEONCOUNT],
            walls: vec![false; DUNGEONCOUNT],
            revealed: vec![false; DUNGEONCOUNT],
            width: DUNGEONWIDTH as i32,
            height: DUNGEONHEIGHT as i32,
            size: DUNGEONCOUNT as i32,
            bloodstains: HashSet::new(),
            depth: _depth,
            ascii: vec![vec![' '; DUNGEONWIDTH]; DUNGEONHEIGHT],
            seed,
        }
    }


    pub fn get_xy(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        self.tiles[self.get_xy(x, y)]
    }
}
impl fmt::Display for Dungeon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dungeon:\n {}x{}\n ", self.width, self.height,)
    }
}



use crate::*;
pub use image::{DynamicImage, GenericImage, RgbaImage};
pub use rand::{Rng, SeedableRng};
pub use rand_chacha::ChaCha20Rng;

pub use wfc::{Context, ForbidInterface, ForbidPattern, PatternId, RunBorrow, Size, Wave, Wrap};
pub use wfc_image::ImagePatterns;
pub use wfc_image::*;

pub const DUNGEONWIDTH: usize = 48;
pub const DUNGEONHEIGHT: usize = 48;
pub const DUNGEONCOUNT: usize = DUNGEONHEIGHT * DUNGEONWIDTH;
pub const MAX_ROOMS: i32 = 30;
const BASE_COLOR: Color = WHITE;
#[derive(PartialEq)]
enum Tileset {
    Numbers,
    Rooms,
    Knots,
    Castle,
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

static TILESET: AtomicRefCell<Tileset> = AtomicRefCell::new(Tileset::Rooms);
static SHOWHISTORY: AtomicRefCell<bool> = AtomicRefCell::new(false);
#[derive(Clone, Copy, Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct GameState {
    pub pixels: Grid<Color>,
    pub image: DynamicImage,
    pub current_dungeon: Dungeon,
    pub dungeon_gen_idx: usize,
    pub dungeon_gen_timer: f32,
    pub dungeon_gen_history: Vec<Dungeon>,
    pub handle: TextureHandle,
}
impl GameState {
    pub fn new(c: &mut EngineContext) -> Self {
        let ratio = screen_width() / screen_height();
        println!("ratio: {}", ratio);
        let width = 200;
        let height = (width as f32 / ratio) as i32;
        let pixels = Grid::filled_with(width, height, |_x, _y| BLACK.alpha(0.0));
        let image = DynamicImage::ImageRgba8(RgbaImage::new(
            pixels.width() as u32,
            pixels.height() as u32,
        ));
        let handle = c
            .texture_creator
            .borrow_mut()
            .handle_from_image("img", &image);
        let current_dungeon = Dungeon::new(1, 123456);
        let dungeon_gen_idx: usize = 0;
        let dungeon_gen_timer: f32 = 0.0;
        let dungeon_gen_history: Vec<Dungeon> = Vec::new();
        GameState {
            pixels,
            image,
            current_dungeon,
            dungeon_gen_idx,
            dungeon_gen_timer,
            dungeon_gen_history,
            handle,
        }
    }
    pub fn generate_map(&mut self, new_depth: i32, seed: u64) {
        self.dungeon_gen_idx = 0;
        self.dungeon_gen_timer = 0.0;
        self.dungeon_gen_history.clear();
        clear_dungeon(self);
        let mut builder = MidpointBuilder::new(new_depth, seed);
        builder.build_dungeon();
        for (x, y, wall) in builder.dungeon.walls.iter() {
            if *wall {
                draw_rect(vec2(x as f32, y as f32 * 2.0), splat(2.), RED.alpha(0.5), 0);
            }
        }
        // for room in builder.rooms.data.iter() {
        //     let [x1, y1, x2, y2] = *room;
        //     draw_rect(vec2(x1 as f32, y1 as f32), vec2((x2 - x1) as f32, (y2 - y1) as f32) , WHITE.alpha(0.5), 0);

        }
    }



pub fn game_update(state: &mut GameState, c: &mut EngineContext) {
    // c.texture_creator
    //     .borrow_mut()
    //     .update_texture(&state.image, state.handle);
    // for (x, y, val) in state.pixels.iter() {
    //     state
    //         .image
    //         .put_pixel(x as u32, y as u32, val.to_image_rgba());
    // }
    clear_background(GRAY) ;
     let viewport = main_camera_mut().world_viewport() / 2.0;

    //pass down to input image
    // egui::Window::new("Tilesets")
    //     .anchor(egui::Align2::RIGHT_CENTER, egui::vec2(10.0, 10.0))
    //     .show(egui(), |ui| {
    //         let mut tileset = TILESET.borrow_mut();
    //         // if ui.radio_value(&mut *tileset, Rooms, "Rooms").clicked() {
    //         //     let path = "../assets/tilesets/Rooms/Roomtileset.png";
    //         //     println!("Loading image: {}", path);
    //         //     state.dungeon.input_image = image::open(path).unwrap();
    //         //     state.dungeon.generate();
    //         // }
    //         // if ui.radio_value(&mut *tileset, Castle, "Castle").clicked() {
    //         //     let path = "../../../assets/tilesets/Castle/Castletileset.png";
    //         //     println!("Loading image: {}", path);
    //         //     state.dungeon.input_image = image::open(path).unwrap();
    //         //     state.dungeon.generate();
    //         }
    //     });

    if is_key_pressed(KeyCode::R) {
        println!("Rerolling dungeon...");
        let seed = rand::thread_rng().gen_range(0..=u64::MAX);
        state.dungeon_gen_idx += 1;
        state
            .dungeon_gen_history
            .push(state.current_dungeon.clone());
        state.generate_map(1, seed);

        // state.dungeon.generate();
    }
}

pub fn clear_dungeon(state: &mut GameState) {
    for (_x, _y, val) in state.pixels.iter_mut() {
        *val = BASE_COLOR.alpha(0.0);
    }
}

pub trait DungeonBuilder {
    fn build_dungeon(&mut self);
    fn spawn_etities(&mut self, hecs: &mut World);
    fn get_dungeon(&self) -> Dungeon;
    fn get_start_position(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Dungeon>;
    fn take_snapshot(&mut self);
}
pub struct MidpointBuilder {
    dungeon: Dungeon,
    start_position: Position,
    depth: i32,
    rooms: Grid<[Position; 2]>,
    dungeon_gen_history: Vec<Dungeon>,
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
        self.rooms.data.clear();
        self.rooms.data.push([Position{x:2, y:2}, Position{x: self.dungeon.width - 5, y:self.dungeon.height - 5}]);

       self.displace();
    }
    fn take_snapshot(&mut self) {
        let show_history = SHOWHISTORY.borrow_mut();
        if *show_history {
            let mut snapshot = self.dungeon.clone();
            for (_x, _y, reveal) in snapshot.revealed.iter_mut() {
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
            rooms: Grid::new(DUNGEONWIDTH as i32, DUNGEONHEIGHT as i32, [Position {x:0,y:0}, Position{x: DUNGEONWIDTH as i32, y: DUNGEONHEIGHT as i32}]),
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
    //todo: use draw_rect to draw room boundaries
    fn draw_room_boundary(&mut self, x1: i32, x2: i32, y1: i32, y2: i32) {
        (x1..=x2).for_each(|x| {
            self.dungeon.walls[(x, y1)] = true;
            self.dungeon.walls[(x, y2)] = true;
        });
        (y1..=y2).for_each(|y| {
            self.dungeon.walls[(x1, y)] = true;
            self.dungeon.walls[(x2, y)] = true;
        });
    }


    fn clear_room(&mut self, x1: i32, x2: i32, y1: i32, y2: i32) {
        (x1..=x2).for_each(|x| {
            self.dungeon.walls[(x, y1)] = false; 
            self.dungeon.walls[(x, y2)] = false;
        });
        (y1..=y2).for_each(|y| {
            self.dungeon.walls[(x1, y)] = false;
            self.dungeon.walls[(x2, y)] = false;
        });
    }
    fn displace(&mut self) {
        let mut rng = ChaCha20Rng::seed_from_u64(self.dungeon.seed);
        // self.rooms.data.push([2, 2, self.dungeon.width - 5, self.dungeon.height - 5]);
        // let mut resolution = (2_f32.powf(self.dungeon.size as f32) - 1.) as i32;
        // let mut resolution = self.dungeon.width.min(self.dungeon.height) / 2 - 1;
        // while resolution >= 1 {
        //     let half_res = resolution / 2;
            // for x in (half_res..self.dungeon.size).step_by(resolution as usize) {
            //     for y in (half_res..self.dungeon.size).step_by(resolution as usize) {
                    // let _top_left = self.dungeon.walls.get(x - half_res, y - half_res);
                    // let _top_right = self.dungeon.walls.get(x + half_res, y - half_res);
                    // let _bottom_left = self.dungeon.walls.get(x - half_res, y + half_res);
                    // let _bottom_right = self.dungeon.walls.get(x + half_res, y + half_res);
                    for  _i in 0..MAX_ROOMS {
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
                    let canidate = [Position{x: x1, y: y1}, Position{x: x2, y: y2}];
                    for other_room in self.rooms.data.iter() {
                        if Self::intersects(canidate, *other_room) {
                            ok = false;
                        }

                    }
                    if ok {
                    if self.is_possible(x1, x2, y1, y2) {
                        self.draw_room_boundary(x1, x2, y1, y2);
                        println!(
                            "canidate room has boundaries of {},{} to {},{}",
                            x1, y1, x2, y2
                        );
                        self.rooms.data.push([Position{x: x1+ 2, y: y1 + 2}, Position{x: x2 - 2, y: y2 - 2}]);
                    }
                    }
                }
            }
            // resolution /= 2;
        // }
    // }
    // // fn normalize(&mut self) {
    //     let mut max = 0.0;
    //     let mut min = 0.0;
    //     for x in 0..self.dungeon.size {
    //     for y in 0..self.dungeon.size {
    //         let val = self.dungeon.tiles.get(x, y);
    //         if val > max {
    //             max = val;
    //         }
    //         if val < min {
    //             min = val;
    //         }
    //     }
    //     }
    //     for x in 0..self.dungeon.size {
    //     for y in 0..self.dungeon.size {
    //         let val = self.dungeon.tiles.get_checked(x, y);
    //         let normalized_val = (val - min) / (max - min);
    //         self.dungeon.tiles.set_checked(x, y, normalized_val);
    //     }
    // }
    //TODO: Should check if is possible to build new room
    fn intersects(canidate: [Position; 2], other: [Position; 2]) -> bool {
        canidate.get(0).unwrap().x <= other.get(1).unwrap().x && canidate.get(1).unwrap().x >= other.get(0).unwrap().x && canidate.get(0).unwrap().y <= other.get(1).unwrap().y && canidate.get(1).unwrap().y >= other.get(0).unwrap().y
            
    }
    fn is_possible(&self, mut x1: i32, mut x2: i32, mut y1: i32, mut y2: i32) -> bool {
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
// struct Forbid {
//     pattern_ids: HashSet<PatternId>,
//     offset: i32,
// }

// impl ForbidPattern for Forbid {
//     fn forbid<W: Wrap, R: Rng>(&mut self, fi: &mut ForbidInterface<W>, rng: &mut R) {
//         let output_size = fi.wave_size();
//         (0..(output_size.width() as i32))
//             .map(|x| Coord::new(x, output_size.height() as i32 - self.offset))
//             .chain(
//                 (0..(output_size.width() as i32))
//                     .map(|y| Coord::new(output_size.width() as i32 - self.offset, y)),
//             )
//             .for_each(|coord| {
//                 self.pattern_ids.iter().for_each(|&pattern_id| {
//                     fi.forbid_all_patterns_except(coord, pattern_id, rng)
//                         .unwrap();
//                 });
//             });
//     }
// }

#[derive(Clone, Debug)]
pub struct Dungeon {
    pub walls: Grid<bool>,
    pub width: i32,
    pub height: i32,
    pub size: i32,
    pub depth: i32,
    pub bloodstains: HashSet<usize>,
    pub revealed: Grid<bool>,
    pub tiles: Grid<TileType>,
    pub seed: u64,
}

impl Dungeon {
    fn new(_depth: i32, seed: u64) -> Self {
        Dungeon {
            // input_image: DynamicImage::new_rgba8(0, 0),
            tiles: Grid::filled_with(DUNGEONWIDTH as i32, DUNGEONHEIGHT as i32, |_x, _y| {
                TileType::Wall
            }),
            walls: Grid::filled_with(DUNGEONWIDTH as i32, DUNGEONHEIGHT as i32, |_x, _y| false),
            revealed: Grid::filled_with(DUNGEONWIDTH as i32, DUNGEONHEIGHT as i32, |_x, _y| false),
            width: DUNGEONWIDTH as i32,
            height: DUNGEONHEIGHT as i32,
            size: DUNGEONCOUNT as i32,
            bloodstains: HashSet::new(),
            depth: _depth,
            seed,
        }
    }
}
// fn generate(&self) -> Result<(), ()> {
//     println!(
//         "generating dungeon of {} by {}, seed: {}",
//         self.width, self.height, self.seed
//     );
//     let mut rng: ChaCha20Rng = rand::SeedableRng::seed_from_u64(self.seed);

//     let image_patterns = ImagePatterns::new(
//         // &self.input_image,
//         NonZeroU32::new(3).unwrap(),
//         &orientation::ALL,
//     );
//     let input_size = image_patterns.grid().size();
//     let output_size: Size = wfc_image::Size::new(DUNGEONWIDTH as u32, DUNGEONHEIGHT as u32);
//     let bottom_right_offset = 3 - (3 / 2);
//     let id_grid = image_patterns.id_grid();
//     let bottom_right_coord = Coord::new(
//         input_size.width() as i32 - bottom_right_offset,
//         input_size.height() as i32 - bottom_right_offset,
//     );
//     let bottom_right_ids = id_grid
//         .get_checked(bottom_right_coord)
//         .iter()
//         .cloned()
//         .collect::<HashSet<_>>();
//     let global_stats = image_patterns.global_stats();
//     let mut wave = Wave::new(output_size);
//     let mut context = Context::new();
//     let result = {
//         let forbid = Forbid {
//             pattern_ids: bottom_right_ids,
//             offset: bottom_right_offset,
//         };
//         let mut run =
//             RunBorrow::new_forbid(&mut context, &mut wave, &global_stats, forbid, &mut rng);
//         run.collapse_retrying(NumTimes(10), &mut rng)
//     };
//     match result {
//         Err(_) => {
//             eprintln!("Too many contradictions!");
//             Err(())
//         }
//         Ok(()) => {
//             image_patterns
//                 .image_from_wave(&wave)
//                 .save("test_output.png")
//                 .unwrap();
//             Ok(())
//         }
//     }
// }

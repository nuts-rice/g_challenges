use crate::*;
pub use image::{DynamicImage, GenericImage, RgbaImage};
pub use rand::{SeedableRng,Rng};
pub use rand_chacha::ChaCha20Rng;

pub use wfc::{Context, ForbidInterface, ForbidPattern, PatternId, RunBorrow, Size, Wave, Wrap};
pub use wfc_image::ImagePatterns;
pub use wfc_image::*;

pub const DUNGEONWIDTH: usize = 120;
pub const DUNGEONHEIGHT: usize = 120;
pub const DUNGEONCOUNT: usize = DUNGEONHEIGHT * DUNGEONWIDTH;
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
    pub dungeon: Dungeon,
    pub pixels: Grid<Color>,
    pub image: DynamicImage,
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
        let dungeon = Dungeon::new(1);
        let dungeon_gen_idx: usize = 0;
        let dungeon_gen_timer: f32 = 0.0;
        let dungeon_gen_history: Vec<Dungeon> = Vec::new();
        GameState {
            dungeon,
            pixels,
            image,
            dungeon_gen_idx,
            dungeon_gen_timer,
            dungeon_gen_history,
            handle,
        }
    }
}
pub fn game_update(state: &mut GameState, c: &mut EngineContext) {
    
    c.texture_creator
        .borrow_mut()
        .update_texture(&state.image, state.handle);
    for (x, y, val) in state.pixels.iter() {
        state
            .image
            .put_pixel(x as u32, y as u32, val.to_image_rgba());
    }
    let _viewport = main_camera_mut().world_viewport();
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
        state.dungeon_gen_idx += 1;
        state.dungeon_gen_history.push(state.dungeon.clone());
        // state.dungeon.generate();
    }
}

pub fn clear_dungeon(state: &mut GameState) {
    for (_x, _y, val) in state.pixels.iter_mut() {
        *val = WHITE.alpha(0.0);
    }
}
pub fn draw_dungeon(_state: &mut GameState) {
    let _viewport = main_camera_mut().world_viewport();
    let _img_dims = vec2(_state.image.width() as f32, _state.image.height() as f32);
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
    rooms: Grid<Vec2>,
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
        self.displace()
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
    fn displace(&mut self) {
        let mut rng = ChaCha20Rng::seed_from_u64(self.dungeon.seed);
        let resolution =
            2_i32.pow(self.dungeon.width as u32 * self.dungeon.height as u32) - 1;
        while resolution >= 1 {
            let half_res = resolution / 2;
            for 

        }
    }
}
struct Forbid {
    pattern_ids: HashSet<PatternId>,
    offset: i32,
}

impl ForbidPattern for Forbid {
    fn forbid<W: Wrap, R: Rng>(&mut self, fi: &mut ForbidInterface<W>, rng: &mut R) {
        let output_size = fi.wave_size();
        (0..(output_size.width() as i32))
            .map(|x| Coord::new(x, output_size.height() as i32 - self.offset))
            .chain(
                (0..(output_size.width() as i32))
                    .map(|y| Coord::new(output_size.width() as i32 - self.offset, y)),
            )
            .for_each(|coord| {
                self.pattern_ids.iter().for_each(|&pattern_id| {
                    fi.forbid_all_patterns_except(coord, pattern_id, rng)
                        .unwrap();
                });
            });
    }
}

#[derive(Clone, Debug)]
pub struct Dungeon {
    pub walls: Grid<bool>,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub bloodstains: HashSet<usize>,
    pub revealed: Grid<bool>,
    pub tiles: Grid<TileType>,
    pub seed: u64,
}

impl Dungeon {
    fn new(_depth: i32) -> Self {
        Dungeon {
            // input_image: DynamicImage::new_rgba8(0, 0),
            tiles: Grid::filled_with(DUNGEONWIDTH as i32, DUNGEONHEIGHT as i32, |_x, _y| {
                TileType::Wall
            }),
            walls: Grid::filled_with(DUNGEONWIDTH as i32, DUNGEONHEIGHT as i32, |_x, _y| false),
            revealed: Grid::filled_with(DUNGEONWIDTH as i32, DUNGEONHEIGHT as i32, |_x, _y| false),
            width: DUNGEONWIDTH as i32,
            height: DUNGEONHEIGHT as i32,
            bloodstains: HashSet::new(),
            depth: _depth,
            seed: 123456789,
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
}

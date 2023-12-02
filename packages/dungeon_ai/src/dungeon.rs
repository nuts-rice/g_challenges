use comfy::Rect;
use ldtk_rust::Project;


pub const DUNGEONWIDTH: i32 = 80;
pub const DUNGEONHEIGHT: i32 = 43;
pub const DUNGEONSIZE: i32 = DUNGEONWIDTH * DUNGEONHEIGHT;
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
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub seed: u64,
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
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn load_map() {
        let _ldtk = Project::load_project("assets/level.ldtk");
    }
}

// impl fmt::Display for Dungeon {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut s = String::new();
//         for y in 0..self.height {
//             for x in 0..self.width {
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

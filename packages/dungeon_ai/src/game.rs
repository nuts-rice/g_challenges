use comfy::epaint::Pos2;

use crate::*;
pub struct Player {
    pub pos: Pos2,
    pub move_timer: f32,
    pub moved: bool,
}
impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Pos2::new(0., 0.),
            move_timer: 0.0,
            moved: false,
        }
    }
}
pub struct GameState {
    pub hecs: World,
    pub map: Dungeon,
    pub mapgen_history: Vec<Dungeon>,
    pub mapgen_idx: usize,
    pub player: Player,
}

pub fn spawn_entities(_world: &mut World, _c: &mut EngineContext) {
    unimplemented!()
}
impl GameState {
    pub fn new(_c: &mut EngineContext) -> Self {
        GameState {
            hecs: World::new(),
            map: Dungeon::new(),
            mapgen_history: Vec::new(),
            mapgen_idx: 0,
            player: Player::new(),
        }
    }

    pub fn run_systems(&mut self, _c: &mut EngineContext) {
        let mut mob_ai = MobAI {};
        mob_ai.run(self, _c);
    }
}
// let filter = env_logger::filter::Filter::("info, wgpu_core=warn, wgpu_hal=warn, dungeon_ai=debug");
// env_logger::filter::Filter::filter("info,wgpu_core=warn,wgpu_hal=warn,dungeon_ai=debug");

pub fn game_update(_state: &mut GameState, _c: &mut EngineContext) {
    let _viewport = main_camera().world_viewport();   
    _state.run_systems(_c);
}

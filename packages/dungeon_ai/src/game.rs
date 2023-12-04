use crate::*;
pub struct Player {}
impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Player {}
    }
}
pub struct GameState {
    pub map: Dungeon,
}
impl GameState {
    pub fn new(_c: &mut EngineContext) -> Self {
        GameState {
            map: Dungeon::new(),
        }
    }
}
// let filter = env_logger::filter::Filter::("info, wgpu_core=warn, wgpu_hal=warn, dungeon_ai=debug");
// env_logger::filter::Filter::filter("info,wgpu_core=warn,wgpu_hal=warn,dungeon_ai=debug");

pub fn game_update(_state: &mut GameState, _c: &mut EngineContext) {
    unimplemented!()
}

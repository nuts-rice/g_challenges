mod ai;
mod components;
mod dungeon;

pub use comfy::*;
pub use components::*;
pub use dungeon::*;

mod game;
pub use ai::*;
pub use game::*;

simple_game!("Rougelike AI demo", GameState, setup, game_update);

fn setup(_state: &mut GameState, _ctx: &mut EngineContext) {
    let world = World::new();
    let mut map = Dungeon::new();
    let mut gs = GameState {
        hecs: world,
        map: map.clone(),
        player: Player::new(),
        mapgen_history: Vec::new(),
        mapgen_idx: 0,
    };
    map.generate_map(1, 123456789);
    let _mob_ai = MobAI {};
    gs.run_systems(_ctx);
}

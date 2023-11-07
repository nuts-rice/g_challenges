pub use comfy::*;
pub use grids::*;
mod dungeon;
pub use crate::dungeon::*;
simple_game!("dungeon generator", GameState, setup, game_update);
fn config() {
    unimplemented!()
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {
    tracy_client::Client::start();
    info!("Setup");
    _state.generate_map(1, 1245345);
}

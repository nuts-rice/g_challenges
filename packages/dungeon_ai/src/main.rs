mod ai;
mod dungeon;
pub use dungeon::*;

pub use comfy::*;

mod game;
pub use ai::*;
pub use game::*;

simple_game!("Rougelike AI demo", GameState, setup, game_update);

fn setup(_state: &mut GameState, _ctx: &mut EngineContext) {
    println!("Hello World!");
}

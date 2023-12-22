use crate::{Dungeon, GameState, MoveMode, Movement};
use comfy::{epaint::Pos2, *};
use specs::Entities;

pub struct MobAI {}
#[derive(Debug)]
struct Mob {
    pub name: String,
    pub position: Pos2,
    pub velocity: Vec2,
    pub move_timer: f32,
    pub move_target: Pos2,
}
impl Mob {
    pub fn new(position: Pos2) -> Self {
        Self {
            position,
            name: "dummy".to_string(),
            velocity: Vec2::ZERO,
            move_timer: 0.0,
            move_target: position,
        }
    }
}
type SystemData = (Entities<'static>, Transform, MoveMode, Dungeon);
impl MobAI {
    pub fn run(&mut self, state: &mut GameState, _c: &EngineContext) {
        for (_entity, (_transform, mode)) in
            world().query::<(&mut Transform, &mut MoveMode)>().iter()
        {
            let mut moved = false;
            let mut move_dir = Vec2::ZERO;
            let move_roll = random_i32(1, 5);
            let _dt = delta();
            let _speed = 1.0;
            match &mut mode.mode {
                Movement::Static => {}
                Movement::Random => {
                    match move_roll {
                        1 => {
                            move_dir.x += 1.;
                            moved = true;
                        }
                        2 => {
                            move_dir.x -= 1.;
                            moved = true;
                        }
                        3 => {
                            move_dir.y += 1.;
                            moved = true;
                        }
                        4 => {
                            move_dir.y -= 1.;
                            moved = true;
                        }
                        _ => (),
                    }
                    if move_dir.x > 0.
                        && move_dir.x < state.map.width - 1.
                        && move_dir.y > 0.
                        && move_dir.y < state.map.height - 1.
                    {
                        let destination = state.map.xy_idx(move_dir.x as i32, move_dir.y as i32);

                        if !state.map.walls[destination] {
                            let idx = state
                                .map
                                .xy_idx(_transform.position.y as i32, _transform.position.y as i32);
                            state.map.walls[idx] = false;
                            _transform.position.x = move_dir.x;
                            _transform.position.y = move_dir.y;
                            state.map.walls[destination] = true;
                        }
                    }
                }
                Movement::RandomWaypoint { path: _ } => {}
            }
        }
    }
}

use crate::Dungeon;
use comfy::*;
pub struct MobAI {}
struct Mob {}

// type SystemData = (WriteExpect<'a, Dungeon>, WriteExpect <'a, rand::rngs::StdRng>, WriteStorage<'a, Position>, Entities<'a>);
impl MobAI {
    fn run(&mut self, mut dungeon: Dungeon) {
        // let (mut dungeon, mut rng, mut position, entities) = data;
        for (_mob, transform) in world().query::<(&mut Transform, &Mob)>().iter() {
            let mut moved = false;
            let mut move_dir = Vec2::ZERO;
            let move_roll = random_i32(1, 5);
            let _dt = delta();
            let _speed = 1.0;
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
                && move_dir.x < dungeon.width - 1.
                && move_dir.y > 0.
                && move_dir.y < dungeon.height - 1.
            {
                let destination = dungeon.xy_idx(move_dir.x as i32, move_dir.y as i32);

                if !dungeon.walls[destination] {
                    let idx = dungeon
                        .xy_idx(transform.0.position.y as i32, transform.0.position.y as i32);
                    dungeon.walls[idx] = false;

                    // transform.0.position.x = x;
                    // transform.0.position.y = y;
                }
            }
        }
    }
}

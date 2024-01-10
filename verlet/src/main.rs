use blobs::*;
use comfy::*;
pub mod components;
pub mod systems;
pub use components::*;

pub mod prelude {
    pub use crate::{components::*, systems::*, VerletState};
}
comfy_game!("Verlet", VerletState);
// pub struct VerletSystem {
//     pub particles: Vec<VerletPoint>,
//     pub constraints: Vec<Constraint>,
// }

pub struct VerletState {
    pub physics: Physics,
    pub friction: f32,
}
impl GameLoop for VerletState {
    fn new(_c: &mut EngineState) -> Self {
        const GRAVITY: Vec2 = vec2(0.0, -9.81);
        Self {
            physics: Physics::new(GRAVITY, false),
            friction: 0.99,
        }
    }

    fn update(&mut self, _c: &mut EngineContext) {
        let gravity = self.physics.gravity * delta();
        let friction = self.friction;

        for (_, (transform, point)) in
            &mut world().query::<(&mut Transform, &mut VerletPoint)>().iter()
        {
            systems::update_point(transform, point, gravity, friction);
        }
    }
}

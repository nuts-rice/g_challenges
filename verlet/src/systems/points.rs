use comfy::{Transform, Vec2};

use crate::components::VerletPoint;

pub fn update_point(transform: &mut Transform, point: &mut VerletPoint, acc: Vec2, friction: f32) {
    let position = transform.position;
    let velocity = point.old_position.map_or(Vec2::ZERO, |pos| position - pos);
    transform.position += velocity * friction + acc;
    point.old_position = Some(position);
}
//Moved to main
//pub fn update_points(constaints: Constraint, c: &EngineContext, ){
//    let gravity = constaints.physics.gravity * delta();
//    let friction = constaints.friction;
//    // for (_, (transform, point)) in &mut world().query::<(&mut Transform, &mut VerletPoint )>().iter() {
//    //     update_point(transform, point, gravity, friction);
//    //     }
//    //TODO: query world for transform and verlet point
//    // for (_, (transform, point) in world().query::<(&mut Transform, &mut VerletPoint)>().iter() {
//    //     update_point(transform, point, acc, friction);
//    // }

//}

use blobs::*;
use comfy::*;
pub mod components;
pub mod systems;
pub use components::*;

pub mod prelude {
    pub use crate::{components::*, systems::*, VerletState};
}

static CANVAS: AtomicRefCell<Canvas> = AtomicRefCell::new(Canvas {});

#[derive(Debug, Clone, Default)]
struct Canvas {}

impl Canvas {
    fn new() -> Self {
        Self {}
    }
}
comfy_game!("Verlet", VerletState);
// pub struct VerletSystem {
//     pub particles: Vec<VerletPoint>,
//     pub constraints: Vec<Constraint>,
// }
pub struct VerletConfig {
    pub constraint_depth: u8,
    pub parallel: bool,
}
pub struct VerletState {
    pub physics: Physics,
    pub friction: f32,
    pub config: VerletConfig,
}
impl GameLoop for VerletState {
    fn new(_c: &mut EngineState) -> Self {
        const GRAVITY: Vec2 = vec2(0.0, -9.81);
        let mut state = Self {
            physics: Physics::new(GRAVITY, false),
            friction: 0.99,
            config: VerletConfig {
                constraint_depth: 1,
                parallel: true,
            },
        };
        let rbd_handle = state.physics.insert_rbd(RigidBodyBuilder::new().build());
        state
            .physics
            .insert_collider_with_parent(ColliderBuilder::new().build(), rbd_handle);

        state
    }

    fn update(&mut self, _c: &mut EngineContext) {
        _c.load_texture_from_bytes(
            "point",
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/point.png")),
        );
        let gravity = self.physics.gravity * delta();
        let friction = self.friction;
        let (_origin_x, _origin_y) = (-120., 120.);
        let (points_x, points_y) = (15, 15);
        let mut _entities: Vec<Entity> = Vec::new();
        for points_x in 0..points_x {
            for points_y in 0..points_y {
                // let mut cmd = commands().spawn((Transform::from(origin_x + (10. * x as f32 ), origin_y + (-10. * y as f32), 0.,)),
                // );
                let x = points_x as f32 * 0.5;
                let y = points_y as f32 * 0.5;
                let rbd_handle = self
                    .physics
                    .insert_rbd(RigidBodyBuilder::new().position(vec2(x, y)).build());
                self.physics
                    .insert_collider_with_parent(ColliderBuilder::new().build(), rbd_handle);
                let _cmd = commands().spawn((
                    Sprite::new("point".to_string(), vec2(0.5, 0.5), 3, WHITE),
                    Transform::position(vec2(x, y)),
                    VerletPoint {
                        position: vec2(x, y),
                        old_position: Some(vec2(x, y)),
                        acceleration: gravity,
                        mass: 1.0,
                        is_pinned: false,
                    },
                                    
                ));
                // components::Constraint{point_a: , point_b: cmd, length: 0.0} ;
                // if points_y == 0 {
                //     commands().insert(cmd, (components::VerletLocked));

                //                       }
                // entities.push(cmd));

                // let point = VerletPoint{position: vec2(x, y), old_position: Some(vec2(x, y)), acceleration: gravity, mass: 1.0, is_pinned: false};
                // world().spawn((point,));
            }
        }
        let _debug = self.physics.debug_data();
        // for body in debug.bodies.iter() {
        // draw_circle(body.transform.translation, 0.1, RED.alpha(0.8), 5);
        // }
        for constraint in self.physics.constraints.iter() {
            // let constraint = components::Constraint{point_a: constraint.  , point_b: constraint.position + constraint.radius, length: constraint.length} ;
            draw_line(
                constraint.position,
                constraint.position + constraint.radius,
                10.,
                WHITE.alpha(0.5),
                5,
            );
        }
        for (_entity, (transform, point)) in
            &mut world().query::<(&mut Transform, &mut VerletPoint)>().iter()
        {
        _entities.push(_entity);
            // spawn_line(_c, _entity, &_entities, 0.5, Some(1));
            systems::update_point(transform, point, gravity, friction);

        }
        for (i, entity) in _entities.iter().enumerate() {
            let above = i.checked_sub(points_x);
            let left = if i % points_x == 0 {
                None
            } else {
                i.checked_sub(1)
            };
            spawn_line(_c, *entity, &_entities, 0.5, above);
            spawn_line(_c, *entity, &_entities, 0.5, left);

        }

        // for (_, (transform, point)) in
        //     &mut world().query::<(&mut Transform, &mut VerletPoint)>().iter()
        // {
        //     systems::update_point(transform, point, gravity, friction);

        // }
    }
}

fn build_cloth() {
    todo!()
}
fn cut_cloth() {
   todo!() 
}

fn spawn_line(
    _c: &mut EngineContext,
    entity: Entity,
    entities: &[Entity],
    length: f32,
    coord: Option<usize>,
) {
    if let Some(i) = coord {
        let other = entities[i];
        commands().spawn((components::Constraint {
            point_a: entity,
            point_b: other,
            length,
        },));
    }
}

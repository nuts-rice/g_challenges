use comfy::*;

#[derive(Debug, Clone)]
pub struct Constraint {
    pub point_a: Entity,
    pub point_b: Entity,
    pub length: f32,
}

impl Constraint {
    pub const fn entities(&self) -> [Entity; 2] {
        [self.point_a, self.point_b]
    }
}

use comfy::*;
#[derive(Debug, Default, Clone, Copy)]
pub struct VerletPoint {
    pub position: Vec2,
    pub old_position: Option<Vec2>,
    pub acceleration: Vec2,
    pub mass: f32,
    pub is_pinned: bool,
}

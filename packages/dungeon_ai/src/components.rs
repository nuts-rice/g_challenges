use crate::*;

#[derive(Clone, Eq, Debug, PartialEq, Hash)]
pub enum Movement {
    Static,
    Random,
    RandomWaypoint { path: Option<Vec<usize>> },
}

#[derive(Clone, Debug)]
pub struct MoveMode {
    pub mode: Movement,
}

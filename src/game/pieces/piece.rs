use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

pub trait Piece {
    fn invalid_moves(&self, pos: &Position, x_diff: i8, y_diff: i8) -> HashSet<Position>;
    fn find_prune_direction(&self, x_diff: i8, y_diff: i8) -> MoveDirection;
    fn move_generation(initial_pos: &Position, dir: MoveDirection) -> HashSet<Position>;
}

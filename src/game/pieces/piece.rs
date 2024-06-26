use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

pub trait Piece {
    // TODO: check standard and capture moves
    // TODO: ^ if the same, ignore
    // TODO: ^ if different, dont compare capture moves?
    fn validate_move(&mut self, old_pos: &Position, new_pos: &Position) -> bool {
        let mut valid = false;
        let moves = self.possible_moves(old_pos);

        for potential_move in moves.iter() {
            if new_pos.x == potential_move.x && new_pos.y == potential_move.y {
                valid = true;
                break;
            }

        }

        return valid;
    }

    fn possible_moves(&mut self, initial_pos: &Position) -> HashSet<Position>;

    fn invalid_moves(&self, pos: &Position, x_diff: i8, y_diff: i8) -> HashSet<Position>;
    fn find_prune_direction(&self, x_diff: i8, y_diff: i8) -> MoveDirection;
    fn prune_moves(initial_pos: &Position, dir: MoveDirection) -> HashSet<Position>;
}

use crate::game::side::Side;
use crate::game::pieces::piece::*;
use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Clone)]
pub struct Pawn {
    pub icon: char,
    pub side: Side,

    first_move: bool,
}

impl Pawn {
    pub fn new(side: Side) -> Pawn {
        let icon = match side {
            Side::WHITE => 'P',
            Side::BLACK => 'p',
        };
        let first_move = true;

        Pawn { icon, side, first_move }
    }
}

impl Piece for Pawn {
    // TODO: return tuple (standard, capture), other pieces has this duplicate, see if can check
    // TODO: ^ see if can check equality for less checks
    fn possible_moves(&mut self, initial_pos: &Position) -> HashSet<Position> {
        let moves = HashSet::new();

        return moves;
    }

    fn find_prune_direction(&self, pos: &Position, x_diff: i8, y_diff: i8) -> MoveDirection {
        let dir = MoveDirection::UP;

        return dir;

    }

    fn invalid_moves(&self, possible_moves: &HashSet<Position>, pos: &Position, x_diff: i8, y_diff: i8) -> HashSet<Position> {
        let invalids = HashSet::new();

        return invalids;
    }
}

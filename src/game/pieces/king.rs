use crate::game::side::Side;
use crate::game::pieces::piece::*;
use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Clone)]
pub struct King {
    pub icon: char,

    pub side: Side,
}

impl King {
    pub fn new(side: Side) -> King {
        let icon = match side {
            Side::WHITE => 'K',
            Side::BLACK => 'k',
        };

        King { icon, side }
    }
}

impl Piece for King {
    fn possible_moves(&mut self, initial_pos: &Position) -> HashSet<Position> {
        let moves = HashSet::new();

        return moves;
    }

    fn find_prune_direction(&self, pos: &Position, x_diff: u8, y_diff: u8) -> MoveDirection {
        let dir = MoveDirection::UP;

        return dir;

    }

    fn invalid_moves(&self, possible_moves: &HashSet<Position>, pos: &Position, x_diff: u8, y_diff: u8) -> HashSet<Position> {
        let invalids = HashSet::new();

        return invalids;
    }
}

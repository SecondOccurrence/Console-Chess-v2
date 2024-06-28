use crate::game::side::Side;
use crate::game::pieces::piece::*;

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
}

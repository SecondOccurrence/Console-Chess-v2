use crate::game::side::Side;
use crate::game::pieces::piece::*;

use std::collections::HashSet;

#[derive(Clone)]
pub struct Knight {
    pub icon: char,

    pub side: Side,
}

impl Knight {
    pub fn new(side: Side) -> Knight {
        let icon = match side {
            Side::WHITE => 'N',
            Side::BLACK => 'n',
        };

        Knight { side, icon }
    }
}

impl Piece for Knight {
    fn possible_moves(&mut self, initial_pos: &Position) -> HashSet<Position> {
        let moves = HashSet::new();

        return moves;
    }
}

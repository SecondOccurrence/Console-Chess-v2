use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Rook {
    pub icon: char,

    pub side: Side,
}

impl Rook {
    pub fn new(side: Side) -> Rook {
        let icon = match side {
            Side::WHITE => 'R',
            Side::BLACK => 'r',
        };

        Rook { icon, side }
    }
}

impl Piece for Rook {
    fn possible_moves(&mut self, initial_pos: &Position) -> Vec<Position> {
        let moves = Vec::new();

        return moves;
    }
}

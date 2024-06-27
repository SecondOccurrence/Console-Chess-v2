use crate::game::side::Side;
use crate::game::pieces::piece::*;

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
    fn possible_moves(&mut self, initial_pos: &Position) -> Vec<Position> {
        let moves = Vec::new();

        return moves;
    }
}

use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Bishop {
    pub icon: char,

    pub side: Side,
}

impl Bishop {
    pub fn new(side: Side) -> Bishop {
        let icon = match side {
            Side::WHITE => 'B',
            Side::BLACK => 'b',
        };

        Bishop { icon, side }
    }
}

impl Piece for Bishop {
    fn possible_moves(&mut self, initial_pos: &Position) -> Vec<Position> {
        let moves = Vec::new();

        return moves;
    }
}

use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Queen {
    pub icon: char,

    pub side: Side,
}

impl Queen {
    pub fn new(side: Side) -> Queen {
        let icon = match side {
            Side::WHITE => 'Q',
            Side::BLACK => 'q',
        };

        Queen { icon, side }
    }
}

impl Piece for Queen {
    fn possible_moves(&mut self, initial_pos: &Position) -> Vec<Position> {
        let moves = Vec::new();

        return moves;
    }
}

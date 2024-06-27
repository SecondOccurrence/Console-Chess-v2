use crate::game::side::Side;
use crate::game::pieces::piece::*;

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
    fn possible_moves(&mut self, initial_pos: &Position) -> Vec<Position> {
        let moves = Vec::new();

        return moves;
    }
}

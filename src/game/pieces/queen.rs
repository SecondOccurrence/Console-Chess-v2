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
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

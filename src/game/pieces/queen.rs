use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Queen {
    pub icon: char,

    side: Side,
}

impl Queen {
    pub fn new(side: Side) -> Queen {
        let icon = match side {
            Side::WHITE => 'q',
            Side::BLACK => 'Q',
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

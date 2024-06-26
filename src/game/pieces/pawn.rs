use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Pawn {
    pub icon: char,

    pub side: Side,
}

impl Pawn {
    pub fn new(side: Side) -> Pawn {
        let icon = match side {
            Side::WHITE => 'P',
            Side::BLACK => 'p',
        };

        Pawn { icon, side }
    }
}

impl Piece for Pawn {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

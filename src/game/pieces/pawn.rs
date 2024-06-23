use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Pawn {
    side: Side,
}

impl Pawn {
    pub fn new(side: Side) -> Pawn {
        Pawn { side }
    }
}

impl Piece for Pawn {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

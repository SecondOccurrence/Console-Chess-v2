use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Rook {
    side: Side,
}

impl Rook {
    pub fn new(side: Side) -> Rook {
        Rook { side }
    }
}

impl Piece for Rook {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

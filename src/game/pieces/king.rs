use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct King {
    side: Side,
}

impl King {
    pub fn new(side: Side) -> King {
        King { side }
    }
}

impl Piece for King {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

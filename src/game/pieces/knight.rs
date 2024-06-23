use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Knight {
    side: Side,
}

impl Knight {
    pub fn new(side: Side) -> Knight {
        Knight { side }
    }
}

impl Piece for Knight {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

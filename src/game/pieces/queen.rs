use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Queen {
    side: Side,
}

impl Queen {
    pub fn new(side: Side) -> Queen {
        Queen { side }
    }
}

impl Piece for Queen {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

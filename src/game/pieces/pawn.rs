use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Pawn {
    side: Side,
    position: Position,
}

impl Pawn {
    pub fn new(side: Side) -> Pawn {
        let position = Position { x: 9, y: 9 };
        Pawn { side, position }
    }
}

impl Piece for Pawn {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

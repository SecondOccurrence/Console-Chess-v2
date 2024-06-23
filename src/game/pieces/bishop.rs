use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Bishop {
    side: Side,
    position: Position,
}

impl Bishop {
    pub fn new(side: Side) -> Bishop {
        let position = Position { x: 9, y: 9 };
        Bishop { side, position }
    }
}

impl Piece for Bishop {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

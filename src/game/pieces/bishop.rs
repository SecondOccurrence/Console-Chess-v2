use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Bishop {
    pub icon: char,

    side: Side,
}

impl Bishop {
    pub fn new(side: Side) -> Bishop {
        let icon = match side {
            Side::WHITE => 'b',
            Side::BLACK => 'B',
        };

        Bishop { icon, side }
    }
}

impl Piece for Bishop {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

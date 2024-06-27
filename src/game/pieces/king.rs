use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct King {
    pub icon: char,

    pub side: Side,
}

impl King {
    pub fn new(side: Side) -> King {
        let icon = match side {
            Side::WHITE => 'K',
            Side::BLACK => 'k',
        };

        King { icon, side }
    }
}

impl Piece for King {
    fn validate_move(old_pos: &Position, new_pos: &Position) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

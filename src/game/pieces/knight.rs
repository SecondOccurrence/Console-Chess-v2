use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Knight {
    pub icon: char,

    side: Side,
}

impl Knight {
    pub fn new(side: Side) -> Knight {
        let icon = match side {
            Side::WHITE => 'n',
            Side::BLACK => 'N',
        };

        Knight { side, icon }
    }
}

impl Piece for Knight {
    fn validate_move(new_move: &str) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}
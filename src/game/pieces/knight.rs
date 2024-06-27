use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Knight {
    pub icon: char,

    pub side: Side,
}

impl Knight {
    pub fn new(side: Side) -> Knight {
        let icon = match side {
            Side::WHITE => 'N',
            Side::BLACK => 'n',
        };

        Knight { side, icon }
    }
}

impl Piece for Knight {
    fn validate_move(old_pos: &Position, new_pos: &Position) -> bool {
        // TODO: generate all possible moves for this piece. Compare with move.
        // TODO: Turn move to Position, use position comparison
        return true;
    }
}

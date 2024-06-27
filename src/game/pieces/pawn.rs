use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Pawn {
    pub icon: char,
    pub side: Side,

    first_move: bool,
}

impl Pawn {
    pub fn new(side: Side) -> Pawn {
        let icon = match side {
            Side::WHITE => 'P',
            Side::BLACK => 'p',
        };
        let first_move = true;

        Pawn { icon, side, first_move }
    }
}

impl Piece for Pawn {
    fn possible_moves(&mut self, initial_pos: &Position) -> Vec<Position> {
        let mut moves = Vec::new();
        let increment;
        if self.side == Side::WHITE {
            increment = 1;
        }
        else {
            increment = -1;
        }

        if self.first_move {
            let (new_x, new_y) = (initial_pos.x, initial_pos.y + (2 * increment));
            let new_move = Position{ x: new_x, y: new_y };
            moves.push(new_move);
            self.first_move = false;
        }

        let (new_x, new_y) = (initial_pos.x, initial_pos.y + increment);
        let new_move = Position{ x: new_x, y: new_y };
        moves.push(new_move);

        return moves;
    }
}

use crate::game::side::Side;
use crate::game::pieces::piece::*;
use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Clone)]
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
    fn move_directions() -> Vec<MoveDirection> {
        return vec![MoveDirection::Up];
    }

    fn find_prune_direction(&self, x_diff: i8, y_diff: i8) -> MoveDirection {
        let dir = MoveDirection::Up;

        return dir;

    }

    fn invalid_moves(&self, pos: &Position, x_diff: i8, y_diff: i8) -> HashSet<Position> {
        let invalids = HashSet::new();

        return invalids;
    }

    fn move_generation(initial_pos: &Position, dir: MoveDirection) -> HashSet<Position> {
        let mut invalids: HashSet<Position> = HashSet::new();

        return invalids;
    }
}

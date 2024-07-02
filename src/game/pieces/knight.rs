use crate::game::side::Side;
use crate::game::pieces::piece::*;
use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Clone)]
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
    fn possible_moves(&mut self, initial_pos: &Position) -> HashSet<Position> {
        let moves = HashSet::new();

        return moves;
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

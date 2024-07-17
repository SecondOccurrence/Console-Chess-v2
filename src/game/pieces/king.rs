use crate::game::side::Side;
use crate::game::pieces::piece::*;
use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Clone)]
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

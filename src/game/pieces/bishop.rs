use crate::game::side::Side;
use crate::game::pieces::piece::*;
use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Clone)]
pub struct Bishop {
    pub icon: char,

    pub side: Side,
}

impl Bishop {
    pub fn new(side: Side) -> Bishop {
        let icon = match side {
            Side::WHITE => 'B',
            Side::BLACK => 'b',
        };

        Bishop { icon, side }
    }
}

impl Piece for Bishop {
    fn possible_moves(&mut self, initial_pos: &Position) -> HashSet<Position> {
        let moves = HashSet::new();

        return moves;
    }

    fn find_prune_direction(&self, pos: &Position, x_diff: i8, y_diff: i8) -> MoveDirection {
        let dir: MoveDirection;
        
        if x_diff < 0 && y_diff < 0 { dir = MoveDirection::UP_RIGHT; }
        else if x_diff < 0 && y_diff > 0 { dir = MoveDirection::DOWN_RIGHT; }
        else if x_diff > 0 && y_diff < 0 { dir = MoveDirection::UP_LEFT; }
        else { dir = MoveDirection::DOWN_LEFT; }

        return dir;

    }

    fn invalid_moves(&self, possible_moves: &HashSet<Position>, pos: &Position, x_diff: i8, y_diff: i8) -> HashSet<Position> {
        let invalids = HashSet::new();
        let dir = self.find_prune_direction(pos, x_diff, y_diff);

        return invalids;
    }
}

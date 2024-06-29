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

    fn invalid_moves(&self, pos: &Position, x_diff: i8, y_diff: i8) -> HashSet<Position> {
        let invalids: HashSet<Position>;
        let dir = self.find_prune_direction(pos, x_diff, y_diff);

        invalids = match dir {
            MoveDirection::UP_RIGHT => Bishop::prune_moves(pos, 8, 8),
            MoveDirection::DOWN_RIGHT => Bishop::prune_moves(pos, 8, 0),
            MoveDirection::UP_LEFT => Bishop::prune_moves(pos, 0, 8),
            MoveDirection::DOWN_LEFT => Bishop::prune_moves(pos, 0, 0),
            _ => HashSet::new(),
        };

        return invalids;
    }

    // TODO: could probably reuse for move generation
    fn prune_moves(initial_pos: &Position, row_max: i8, col_max: i8) -> HashSet<Position> {
        let (row_start, row_end) = (initial_pos.x, row_max);
        let (col_start, col_end) = (initial_pos.y, col_max);
        
        let row_range = if row_start <= row_end {
            row_start..row_end
        }
        else {
            row_end..row_start
        };

        let col_range = if col_start <= col_end {
            col_start..col_end
        }
        else {
            col_end..col_start
        };

        let mut invalids: HashSet<Position> = HashSet::new();
        for row in row_range {
            for col in col_range.clone() {
                let invalid_move = Position { x: row, y: col };
                invalids.insert(invalid_move);

            }
        }

        return invalids;
    }
}

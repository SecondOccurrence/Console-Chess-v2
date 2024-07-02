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

    fn find_prune_direction(&self, x_diff: i8, y_diff: i8) -> MoveDirection {
        let dir: MoveDirection;
        
        if x_diff < 0 && y_diff < 0 { dir = MoveDirection::UpRight; }
        else if x_diff < 0 && y_diff > 0 { dir = MoveDirection::DownRight; }
        else if x_diff > 0 && y_diff < 0 { dir = MoveDirection::UpLeft; }
        else { dir = MoveDirection::DownLeft; }

        return dir;
    }

    fn invalid_moves(&self, pos: &Position, x_diff: i8, y_diff: i8) -> HashSet<Position> {
        let invalids: HashSet<Position>;
        let dir = self.find_prune_direction(x_diff, y_diff);

        invalids = Bishop::move_generation(pos, dir);

        return invalids;
    }

    fn move_generation(initial_pos: &Position, dir: MoveDirection) -> HashSet<Position> {
        let mut x_limit = 8 as i8;
        let mut x_increment = 1 as i8;
        if dir == MoveDirection::UpLeft || dir == MoveDirection::DownLeft {
            x_limit = -1;
            x_increment = -1;
        }

        let mut y_limit = 8 as i8;
        let mut y_increment = 1 as i8;
        if dir == MoveDirection::DownLeft || dir == MoveDirection::DownRight {
            y_limit = -1;
            y_increment = -1;
        }

        let mut invalids: HashSet<Position> = HashSet::new();
        let mut invalid_pos = Position { x: (*initial_pos).x + x_increment, y: (*initial_pos).y + y_increment };
        while invalid_pos.x != x_limit && invalid_pos.y != y_limit {
            invalids.insert(invalid_pos);

            invalid_pos.x += x_increment;
            invalid_pos.y += y_increment;
        }

        return invalids;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_direction() {
        let bishop = Bishop::new(Side::WHITE);

        let dir1 = bishop.find_prune_direction(-1, -1);
        let dir2 = bishop.find_prune_direction(1, -1);
        let dir3 = bishop.find_prune_direction(-1, 1);
        let dir4 = bishop.find_prune_direction(1, 1);

        assert_eq!(dir1, MoveDirection::UpRight, "Expected UpLeft, received: {:?}", dir1);
        assert_eq!(dir2, MoveDirection::UpLeft, "Expected UpRight, received: {:?}", dir2);
        assert_eq!(dir3, MoveDirection::DownRight, "Expected DownLeft, received: {:?}", dir3);
        assert_eq!(dir4, MoveDirection::DownLeft, "Expected DownRight, received: {:?}", dir4);
    }

    #[test]
    fn test_move_pruning() {
        let collision_piece = Position { x: 4, y: 4 };

        {
            let moves = Bishop::move_generation(&collision_piece, MoveDirection::UpRight);
            let positions: [Position; 3] = [
                Position { x: 5, y: 5 },
                Position { x: 6, y: 6 },
                Position { x: 7, y: 7 },
            ];
            moves_exist_check(&moves, &collision_piece, &positions);
        }

        {
            let moves = Bishop::move_generation(&collision_piece, MoveDirection::UpLeft);
            let positions: [Position; 3] = [
                Position { x: 3, y: 5 },
                Position { x: 2, y: 6 },
                Position { x: 1, y: 7 },
            ];
            moves_exist_check(&moves, &collision_piece, &positions);
        }

        {
            let moves = Bishop::move_generation(&collision_piece, MoveDirection::DownRight);
            let positions: [Position; 3] = [
                Position { x: 5, y: 3 },
                Position { x: 6, y: 2 },
                Position { x: 7, y: 1 },
            ];
            moves_exist_check(&moves, &collision_piece, &positions);
        }

        {
            let moves = Bishop::move_generation(&collision_piece, MoveDirection::DownLeft);
            let positions: [Position; 4] = [
                Position { x: 3, y: 3 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 0 },
            ];
            moves_exist_check(&moves, &collision_piece, &positions);
        }
    }

    fn moves_exist_check(moves: &HashSet<Position>, collision_piece: &Position, expected_positions: &[Position]) {
        assert_eq!(moves.len(), expected_positions.len(), "For position ({},{}): Expected moves length: {}. Received moves length: {}",
            collision_piece.x, collision_piece.y, expected_positions.len(), moves.len());

        for expected_pos in expected_positions.iter() {
            assert!(moves.contains(expected_pos), "Expected possible move set to contain position ({},{}), but does not.", expected_pos.x, expected_pos.y);
        }
    }
}

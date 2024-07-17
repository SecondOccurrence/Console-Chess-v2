use crate::game::side::Side;
use crate::game::pieces::piece::*;
use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Clone)]
pub struct Rook {
    pub icon: char,

    pub side: Side,
}

impl Rook {
    pub fn new(side: Side) -> Rook {
        let icon = match side {
            Side::WHITE => 'R',
            Side::BLACK => 'r',
        };

        Rook { icon, side }
    }
}

impl Piece for Rook {
    fn move_directions() -> Vec<MoveDirection> {
        return vec![
            MoveDirection::Up,
            MoveDirection::Down,
            MoveDirection::Left,
            MoveDirection::Right,
        ];
    }

    fn find_prune_direction(&self, x_diff: i8, y_diff: i8) -> MoveDirection {
        let dir: MoveDirection;

        if x_diff != 0 {
            assert_eq!(y_diff, 0, "Rook moving horizontally. Expected difference in y to be 0, received: '{}'", y_diff);
            if x_diff < 0 { dir = MoveDirection::Right; }
            else { dir = MoveDirection::Left; }
        }
        else if y_diff != 0 {
            assert_eq!(x_diff, 0,  "Rook moving vertically. Expected difference in x to be 0, received: '{}'", x_diff);
            if y_diff < 0 { dir = MoveDirection::Up; }
            else { dir = MoveDirection::Down; }
        }
        else {
            panic!("Unexpected condition: x_diff and y_diff are both 0");
        }

        return dir;
    }

    fn invalid_moves(&self, pos: &Position, x_diff: i8, y_diff: i8) -> HashSet<Position> {
        let invalids: HashSet<Position>;
        let dir = self.find_prune_direction(x_diff, y_diff);

        invalids = Rook::move_generation(pos, dir);

        return invalids;
    }


    fn move_generation(initial_pos: &Position, dir: MoveDirection) -> HashSet<Position> {
        let x_limit: i8;
        let x_increment: i8;
        let y_limit: i8;
        let y_increment: i8;

        if dir == MoveDirection::Up {
            x_limit = 0;
            x_increment = 0;
            y_limit = 7;
            y_increment = 1;
        }
        else if dir == MoveDirection::Down {
            x_limit = 0;
            x_increment = 0;
            y_limit = 0;
            y_increment = -1;
        }
        else if dir == MoveDirection::Left {
            x_limit = 0;
            x_increment = -1;
            y_limit = 0;
            y_increment = 0;
        }
        else if dir == MoveDirection::Right {
            x_limit = 7;
            x_increment = 1;
            y_limit = 0;
            y_increment = 0;
        }
        else {
            panic!("Received move direction that cannot be performed by a rook");
        }

        let mut moves: HashSet<Position> = HashSet::new();
        let mut move_pos = Position { x: (*initial_pos).x + x_increment, y: (*initial_pos).y + y_increment };

        loop {
            moves.insert(move_pos);

            if move_pos.x == x_limit || move_pos.y == y_limit {
                break;
            }

            move_pos.x += x_increment;
            move_pos.y += y_increment;
        }
        
        return moves;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_direction() {
        let rook = Rook::new(Side::WHITE);

        let dir1 = rook.find_prune_direction(4, 0);
        let dir2 = rook.find_prune_direction(-4, 0);
        let dir3 = rook.find_prune_direction(0, 4);
        let dir4 = rook.find_prune_direction(0, -4);

        assert_eq!(dir1, MoveDirection::Left, "Expected Left, received: {:?}", dir1);
        assert_eq!(dir2, MoveDirection::Right, "Expected Right, received: {:?}", dir2);
        assert_eq!(dir3, MoveDirection::Down, "Expected Down, received: {:?}", dir3);
        assert_eq!(dir4, MoveDirection::Up, "Expected Up, received: {:?}", dir4);
    }

    #[test]
    #[should_panic]
    fn invalid_move_direction_x() {
        let rook = Rook::new(Side::WHITE);
        let _ = rook.find_prune_direction(-4, -4);
    }

    #[test]
    #[should_panic]
    fn invalid_move_direction_y() {
        let rook = Rook::new(Side::WHITE);
        let _ = rook.find_prune_direction(4, 4);
    }

    #[test]
    #[should_panic]
    fn undefined_move_direction() {
        let rook = Rook::new(Side::WHITE);
        let _ = rook.find_prune_direction(0, 0);
    }

    #[test]
    fn move_generation() {
        let piece_pos = Position { x: 4, y: 4 };

        {
            let moves = Rook::move_generation(&piece_pos, MoveDirection::Up);
            let positions: [Position; 3] = [
                Position { x: 4, y: 5 },
                Position { x: 4, y: 6 },
                Position { x: 4, y: 7 },
            ];
            moves_exist_check(&moves, &piece_pos, &positions);
        }

        {
            let moves = Rook::move_generation(&piece_pos, MoveDirection::Down);
            let positions: [Position; 4] = [
                Position { x: 4, y: 3 },
                Position { x: 4, y: 2 },
                Position { x: 4, y: 1 },
                Position { x: 4, y: 0 },
            ];
            moves_exist_check(&moves, &piece_pos, &positions);
        }

        {
            let moves = Rook::move_generation(&piece_pos, MoveDirection::Left);
            let positions: [Position; 4] = [
                Position { x: 3, y: 4 },
                Position { x: 2, y: 4 },
                Position { x: 1, y: 4 },
                Position { x: 0, y: 4 },
            ];
            moves_exist_check(&moves, &piece_pos, &positions);
        }

        {
            let moves = Rook::move_generation(&piece_pos, MoveDirection::Right);
            let positions: [Position; 3] = [
                Position { x: 5, y: 4 },
                Position { x: 6, y: 4 },
                Position { x: 7, y: 4 },
            ];
            moves_exist_check(&moves, &piece_pos, &positions);
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

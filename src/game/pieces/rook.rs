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
            if x_diff < 0 { dir = MoveDirection::Left; }
            else { dir = MoveDirection::Right; }
        }
        else if y_diff != 0 {
            assert_eq!(x_diff, 0,  "Rook moving vertically. Expected difference in x to be 0, received: '{}'", x_diff);
            if y_diff < 0 { dir = MoveDirection::Down; }
            else { dir = MoveDirection::Up; }
        }
        else {
            panic!("Unexpected condition: x_diff and y_diff are both 0");
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_direction() {
        let rook = Rook::new(Side::WHITE);

        let dir1 = rook.find_prune_direction(-4, 0);
        let dir2 = rook.find_prune_direction(4, 0);
        let dir3 = rook.find_prune_direction(0, -4);
        let dir4 = rook.find_prune_direction(0, 4);

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
}

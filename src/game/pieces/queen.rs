use crate::game::side::Side;
use crate::game::pieces::piece::*;
use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Clone)]
pub struct Queen {
    pub icon: char,

    pub side: Side,
}

impl Queen {
    pub fn new(side: Side) -> Queen {
        let icon = match side {
            Side::WHITE => 'Q',
            Side::BLACK => 'q',
        };

        Queen { icon, side }
    }
}

impl Piece for Queen {
    fn move_directions() -> Vec<MoveDirection> {
        return vec![
            MoveDirection::Up,
            MoveDirection::Down,
            MoveDirection::Left,
            MoveDirection::Right,
            MoveDirection::UpLeft,
            MoveDirection::UpRight,
            MoveDirection::DownLeft,
            MoveDirection::DownRight
        ];
    }

    fn find_prune_direction(&self, x_diff: i8, y_diff: i8) -> MoveDirection {
        let dir: MoveDirection;

        if x_diff == 0 && y_diff < 0 { dir = MoveDirection::Up; }
        else if x_diff == 0 && y_diff > 0 { dir = MoveDirection::Down; }
        else if y_diff == 0 && x_diff > 0 { dir = MoveDirection::Left; }
        else if y_diff == 0 && x_diff < 0 { dir = MoveDirection::Right; }
        else if x_diff > 0 && y_diff < 0 { dir = MoveDirection::UpLeft; }
        else if x_diff < 0 && y_diff < 0 { dir = MoveDirection::UpRight; }
        else if x_diff > 0 && y_diff > 0 { dir = MoveDirection::DownLeft; }
        else if x_diff < 0 && y_diff > 0 { dir = MoveDirection::DownRight; }
        else { panic!("Queen direction unexpected behaviour. Should not reach here ever"); }

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
        let queen = Queen::new(Side::WHITE);

        let dir1 = queen.find_prune_direction(0, -1);
        let dir2 = queen.find_prune_direction(0, 1);
        let dir3 = queen.find_prune_direction(1, 0);
        let dir4 = queen.find_prune_direction(-1, 0);
        let dir5 = queen.find_prune_direction(1, -1);
        let dir6 = queen.find_prune_direction(-1, -1);
        let dir7 = queen.find_prune_direction(1, 1);
        let dir8 = queen.find_prune_direction(-1, 1);

        assert_eq!(dir1, MoveDirection::Up, "Expected Up, received: {:?}", dir1);
        assert_eq!(dir2, MoveDirection::Down, "Expected Down, received: {:?}", dir2);
        assert_eq!(dir3, MoveDirection::Left, "Expected Left, received: {:?}", dir3);
        assert_eq!(dir4, MoveDirection::Right, "Expected Right, received: {:?}", dir4);
        assert_eq!(dir5, MoveDirection::UpLeft, "Expected UpLeft, received: {:?}", dir5);
        assert_eq!(dir6, MoveDirection::UpRight, "Expected UpRight, received: {:?}", dir6);
        assert_eq!(dir7, MoveDirection::DownLeft, "Expected DownLeft, received: {:?}", dir7);
        assert_eq!(dir8, MoveDirection::DownRight, "Expected DownRight, received: {:?}", dir8);
    }

}

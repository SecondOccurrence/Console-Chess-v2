#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

pub trait Piece {
    fn validate_move(&mut self, old_pos: &Position, new_pos: &Position) -> bool {
        let mut valid = false;
        let moves = self.possible_moves(old_pos);

        for potential_move in moves.iter() {
            if new_pos.x == potential_move.x && new_pos.y == potential_move.y {
                valid = true;
                break;
            }

        }

        return valid;
    }

    fn possible_moves(&mut self, initial_pos: &Position) -> Vec<Position>;
}

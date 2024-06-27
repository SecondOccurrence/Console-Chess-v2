#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

pub trait Piece {
    fn validate_move(old_pos: &Position, new_pos: &Position) -> bool;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

pub trait Piece {
    fn validate_move(new_move: &str) -> bool;
}

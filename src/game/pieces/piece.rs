pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub trait Piece {
    fn validate_move(new_move: &str) -> bool;
}

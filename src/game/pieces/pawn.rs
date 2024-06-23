use crate::game::side::Side;
use crate::game::pieces::piece::*;

pub struct Pawn {
    side: Side,
    position: Position,
}

impl Piece for Pawn {}

use crate::game::pieces::king::King;
use crate::game::pieces::queen::Queen;
use crate::game::pieces::rook::Rook;
use crate::game::pieces::bishop::Bishop;
use crate::game::pieces::knight::Knight;
use crate::game::pieces::pawn::Pawn;

pub enum PieceType {
    King(King),
    Queen(Queen),
    Rook(Rook),
    Bishop(Bishop),
    Knight(Knight),
    Pawn(Pawn),
}

impl PieceType {
    pub fn icon(&self) -> char {
        let character = match self {
            PieceType::King(king) => king.icon,
            PieceType::Queen(queen) => queen.icon,
            PieceType::Rook(rook) => rook.icon,
            PieceType::Bishop(bishop) => bishop.icon,
            PieceType::Knight(knight) => knight.icon,
            PieceType::Pawn(pawn) => pawn.icon,
        };

        return character;
    }
}

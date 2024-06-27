use crate::game::pieces::piece::Piece;

use crate::game::pieces::king::King;
use crate::game::pieces::queen::Queen;
use crate::game::pieces::rook::Rook;
use crate::game::pieces::bishop::Bishop;
use crate::game::pieces::knight::Knight;
use crate::game::pieces::pawn::Pawn;

use crate::game::pieces::Position;
use crate::game::side::Side;

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

    pub fn convert(icon: char) -> Option<PieceType> {
        match icon {
            'K' => Some(PieceType::King(King::new(Side::WHITE))),
            'k' => Some(PieceType::King(King::new(Side::BLACK))),
            'Q' => Some(PieceType::Queen(Queen::new(Side::WHITE))),
            'q' => Some(PieceType::Queen(Queen::new(Side::BLACK))),
            'R' => Some(PieceType::Rook(Rook::new(Side::WHITE))),
            'r' => Some(PieceType::Rook(Rook::new(Side::BLACK))),
            'B' => Some(PieceType::Bishop(Bishop::new(Side::WHITE))),
            'b' => Some(PieceType::Bishop(Bishop::new(Side::BLACK))),
            'N' => Some(PieceType::Knight(Knight::new(Side::WHITE))),
            'n' => Some(PieceType::Knight(Knight::new(Side::BLACK))),
            'P' => Some(PieceType::Pawn(Pawn::new(Side::WHITE))),
            'p' => Some(PieceType::Pawn(Pawn::new(Side::BLACK))),
            _ => None,
        }
    }

    pub fn side(&self) -> Side {
        let s = match self {
            PieceType::King(king) => king.side,
            PieceType::Queen(queen) => queen.side,
            PieceType::Rook(rook) => rook.side,
            PieceType::Bishop(bishop) => bishop.side,
            PieceType::Knight(knight) => knight.side,
            PieceType::Pawn(pawn) => pawn.side,
        };

        return s;
    }

    pub fn validate_move(&self, old_pos: &Position, new_pos: &Position) -> bool {
        let valid: bool;
        match self {
            PieceType::King(_) => valid = King::validate_move(old_pos, new_pos),
            PieceType::Queen(_) => valid = Queen::validate_move(old_pos, new_pos),
            PieceType::Rook(_) => valid = Rook::validate_move(old_pos, new_pos),
            PieceType::Bishop(_) => valid = Bishop::validate_move(old_pos, new_pos),
            PieceType::Knight(_) => valid = Knight::validate_move(old_pos, new_pos),
            PieceType::Pawn(_) => valid = Pawn::validate_move(old_pos, new_pos),
        }

        return valid;
    }
}

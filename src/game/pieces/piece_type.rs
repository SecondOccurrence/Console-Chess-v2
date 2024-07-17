use crate::game::pieces::piece::Piece;

use crate::game::pieces::king::King;
use crate::game::pieces::queen::Queen;
use crate::game::pieces::rook::Rook;
use crate::game::pieces::bishop::Bishop;
use crate::game::pieces::knight::Knight;
use crate::game::pieces::pawn::Pawn;

use crate::game::pieces::Position;
use crate::game::side::Side;
use crate::game::move_direction::MoveDirection;

use std::collections::HashSet;

#[derive(Clone)]
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

    pub fn move_directions(&self) -> Vec<MoveDirection> {
        return match self {
            PieceType::King(_) => King::move_directions(),
            PieceType::Queen(_) => Queen::move_directions(),
            PieceType::Rook(_) => Rook::move_directions(),
            PieceType::Bishop(_) => Bishop::move_directions(),
            PieceType::Knight(_) => Bishop::move_directions(),
            PieceType::Pawn(_) => Pawn::move_directions(),
        };
    }

    pub fn invalid_moves(&self, pos: &Position, x_diff: i8, y_diff: i8) -> HashSet<Position> {
        return match self {
            PieceType::King(king) => king.invalid_moves(pos, x_diff, y_diff),
            PieceType::Queen(queen) => queen.invalid_moves(pos, x_diff, y_diff),
            PieceType::Rook(rook) => rook.invalid_moves(pos, x_diff, y_diff),
            PieceType::Bishop(bishop) => bishop.invalid_moves(pos, x_diff, y_diff),
            PieceType::Knight(knight) => knight.invalid_moves(pos, x_diff, y_diff),
            PieceType::Pawn(pawn) => pawn.invalid_moves(pos, x_diff, y_diff),
        };
    }

    pub fn generate_moves(&self, piece_pos: &Position, generate_dir: MoveDirection) -> HashSet<Position> {
        return match self {
            PieceType::King(_) => King::move_generation(piece_pos, generate_dir),
            PieceType::Queen(_) => Queen::move_generation(piece_pos, generate_dir),
            PieceType::Rook(_) => Rook::move_generation(piece_pos, generate_dir),
            PieceType::Bishop(_) => Bishop::move_generation(piece_pos, generate_dir),
            PieceType::Knight(_) => Knight::move_generation(piece_pos, generate_dir),
            PieceType::Pawn(_) => Pawn::move_generation(piece_pos, generate_dir),
        }
    }
}

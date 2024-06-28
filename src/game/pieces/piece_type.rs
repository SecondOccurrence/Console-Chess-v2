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

    pub fn validate_move(&mut self, old_pos: &Position, new_pos: &Position) -> bool {
        let valid: bool;
        match self {
            PieceType::King(king) => valid = king.validate_move(old_pos, new_pos),
            PieceType::Queen(queen) => valid = queen.validate_move(old_pos, new_pos),
            PieceType::Rook(rook) => valid = rook.validate_move(old_pos, new_pos),
            PieceType::Bishop(bishop) => valid = bishop.validate_move(old_pos, new_pos),
            PieceType::Knight(knight) => valid = knight.validate_move(old_pos, new_pos),
            PieceType::Pawn(pawn) => valid = pawn.validate_move(old_pos, new_pos),
        }

        return valid;
    }

    pub fn possible_moves(&mut self, pos: &Position) -> HashSet<Position> {
        return match self {
            PieceType::King(king) => king.possible_moves(pos),
            PieceType::Queen(queen) => queen.possible_moves(pos),
            PieceType::Rook(rook) => rook.possible_moves(pos),
            PieceType::Bishop(bishop) => bishop.possible_moves(pos),
            PieceType::Knight(knight) => knight.possible_moves(pos),
            PieceType::Pawn(pawn) => pawn.possible_moves(pos),
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
}

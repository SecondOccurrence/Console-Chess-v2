use crate::game::pieces::{ Position, PieceType };

use std::collections::HashMap;

pub struct ChessBoard {
    board: [[char; 8]; 8],  
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let board: [[char; 8]; 8] = [
            ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        ];
        ChessBoard { board }
    }

    pub fn get_piece(&self, row: usize, column: usize) -> char {
        assert!(row <= 7, "Board piece access: expected row <= 7");
        assert!(column <= 7, "Board piece access: expected column <= 7");
        return self.board[row][column];
    }

    pub fn update_board(&mut self, pieces_a: &HashMap<Position, PieceType>, pieces_b: &HashMap<Position, PieceType>) {
        for row in 0..8 {
            for column in 0..8 {
                if let Some(piece) = pieces_a.get(&Position { x: row as i8, y: column as i8 }) {
                    self.board[column][row] = piece.icon();
                }
                else if let Some(piece) = pieces_b.get(&Position { x: row as i8, y: column as i8 }) {
                    self.board[column][row] = piece.icon();
                }
                else {
                    self.board[column][row] = '.';
                }

            }
        }
    }

    pub fn clear(&mut self) {
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = '.';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_piece_retrieval() {
        let board = ChessBoard::new();
        _ = board.get_piece(0, 0);
        _ = board.get_piece(3, 4);
        _ = board.get_piece(7, 7);
    }

    #[test]
    #[should_panic]
    fn invalid_piece_retrieval_x() {
        let board = ChessBoard::new();
        _ = board.get_piece(8, 4);
    }

    #[test]
    #[should_panic]
    fn invalid_piece_retrieval_y() {
        let board = ChessBoard::new();
        _ = board.get_piece(4, 8);
    }
}

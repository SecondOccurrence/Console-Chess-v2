pub struct ChessBoard {
    board: [[char; 8]; 8],  
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let board: [[char; 8]; 8] = [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            ['R', 'N', 'B', 'K', 'Q', 'B', 'N', 'R'],
        ];
        ChessBoard { board }
    }

    pub fn get_piece(&self, row: usize, column: usize) -> char {
        assert!(row <= 7, "Board piece access: expected row <= 7");
        assert!(column <= 7, "Board piece access: expected column <= 7");
        return self.board[row][column];
    }
}

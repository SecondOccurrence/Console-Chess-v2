use crate::game::side::Side;
use crate::game::player::Player;
use crate::game::chess_board::ChessBoard;

pub struct GameManager {
    current_side: Side,   
    chess_board: ChessBoard,
    player1: Player,
    player2: Player,
}

impl GameManager {
    pub fn new() -> GameManager {
        let current_side = Side::WHITE;
        let chess_board = ChessBoard::new();
        let player1 = Player::new(Side::WHITE);
        let player2 = Player::new(Side::BLACK);
        GameManager { current_side, chess_board, player1, player2 }
    }

    pub fn run(&mut self) -> bool {
        self.display_board();

        if self.current_side == Side::WHITE {
            println!("White Move:");
            _ = self.player1.move_input();
        }
        else {
            println!("Black Move:");
            _ = self.player2.move_input();
        }

        self.current_side.switch();
        print!("{}[2J", 27 as char);
        print!("{}[1;1H", 27 as char);

        return false;
    }

    fn display_board(&self) {
        println!("          BLACK");
        println!("   +-----------------+");
        for row_itr in 1..9 {
            let board_row = (row_itr as i32 - 8).abs() as usize;

            let mut line = " ".to_string() + &(board_row + 1).to_string();

            for column_itr in 0..10 {
                if column_itr == 0 || column_itr == 9 {
                    line.push_str(" |");
                }
                else {
                    let board_column = (column_itr as i32 - 8).abs() as usize;

                    line.push(' ');
                    line.push(self.chess_board.get_piece(board_row, board_column));
                }
            }
            println!("{}", line);
        }
        println!("   +-----------------+");
        println!("     a b c d e f g h");
        println!("          WHITE");
    }
}

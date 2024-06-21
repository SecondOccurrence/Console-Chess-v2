mod chess_board;
mod player;

use chess_board::ChessBoard;
use player::Player;

#[derive(PartialEq)]
pub enum Side {
    WHITE,
    BLACK,
}

impl Side {
    fn switch(&mut self) {
        *self = match *self {
            Side::WHITE => Side::BLACK,
            Side::BLACK => Side::WHITE,
        };
    }
}

pub struct GameManager {
    side: Side,   
    chess_board: ChessBoard,
    player1: Player,
    player2: Player,
}

impl GameManager {
    pub fn new(side: Side) -> GameManager {
        let chess_board = ChessBoard::new();
        let player1 = Player::new();
        let player2 = Player::new();
        GameManager { side, chess_board, player1, player2 }
    }

    pub fn run(&mut self) -> bool {
        self.display_board();

        if self.side == Side::WHITE {
            _ = self.player1.move_input();
        }
        else {
            _ = self.player2.move_input();
        }

        // TODO: feature: add user input for move

        self.side.switch();

        true
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

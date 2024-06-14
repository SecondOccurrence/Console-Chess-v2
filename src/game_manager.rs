mod chess_board;

use chess_board::ChessBoard;

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
}

impl GameManager {
    pub fn new(side: Side) -> GameManager {
        let chess_board = ChessBoard::new();
        GameManager { side, chess_board }
    }

    pub fn run(&mut self) -> bool {
        self.display_board();

        if self.side == Side::WHITE {
            println!("White turn.");
        }
        else {
            println!("Black turn.");
        }
        // TODO: feature: add user input for move

        self.side.switch();

        true
    }

    fn display_board(&self) {
        println!("           BLACK");
        println!("   +-----------------+");
        for row_num in 0..8 {
            let mut line = " ".to_string() + &row_num.to_string();

            for column in 0..10 {
                if column == 0 || column == 9 {
                    line.push_str(" |");

                }
                else {
                    line.push_str(" .");
                }
            }
            println!("{}", line);
            

        }
        println!("   +-----------------+");
        println!("     a b c d e f g h");
        println!("          BLACK");
    }
}

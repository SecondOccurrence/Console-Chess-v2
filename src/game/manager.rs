use crate::game::side::Side;
use crate::game::player::Player;
use crate::game::chess_board::ChessBoard;
use crate::game::menu::MenuFunctions;

use std::io;
use std::vec;

pub struct GameManager {
    current_side: Side,   
    chess_board: ChessBoard,
    players: [Player; 2],
}

impl GameManager {
    pub fn new() -> GameManager {
        let current_side = Side::WHITE;
        let chess_board = ChessBoard::new();
        let player1 = Player::new(Side::WHITE);
        let player2 = Player::new(Side::BLACK);
        GameManager { current_side, chess_board, players: [player1, player2] }
    }

    pub fn run(&mut self) -> bool {
        self.display_board();

        let side_index = self.current_side.to_index();

        if self.current_side == Side::WHITE {
            println!("White Move:");
        }
        else {
            println!("Black Move:");
        }

        let (initial_position, result_position) = self.players[side_index].move_input();
        // returns -2 if user wishes to enter the menu
        if initial_position.x != -2 {
            if self.players[side_index].piece_at_coord(&result_position) {
                // TODO: update piece counter as piece has been taken             
            }

            self.players[side_index].apply_move(&initial_position, &result_position);

            self.current_side.switch();
            self.chess_board.update_board(self.players[0].pieces(), self.players[1].pieces());
        }
        else {
            self.show_menu();

            loop {
                let mut option = String::new();
                io::stdin().read_line(&mut option).expect("Failed to read move");
                option = option.trim().to_string();

                self.perform_command(&option);

                if option == "exit" {
                    break;
                }
            }
        }

        // clear the console screen through ANSI codes
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

            line.push_str(" | ");
            for column_itr in 0..8 {
                let piece = self.chess_board.get_piece(board_row, column_itr);
                line.push(piece);
                line.push(' ');
            }
            line.push('|');
            println!("{}", line);
        }
        println!("   +-----------------+");
        println!("     a b c d e f g h");
        println!("          WHITE");
    }
}

impl MenuFunctions for GameManager {
    fn show_menu(&self) {
        println!("-- menu --");
        println!("enter choice");
    }

    fn perform_command(&self, option: &str) {
        match option {
            "help" => self.help_menu(),
            "pieces" => self.show_pieces_count(),
            "exit" => println!("Exiting menu.."),
            _ => println!("'{}' is not a valid option", option),
        }
    }

    fn help_menu(&self) {
        // TODO: add new print for each available command
    }

    // TODO: turn into game summary function "sum"
    fn show_pieces_count(&self) {
        println!("\nRemaining Pieces:");
        let white_pieces = self.players[0].pieces();
        let white_tally = self.tally_pieces(white_pieces);
        println!("White side: {:?}", white_tally);

        let black_pieces = self.players[1].pieces();
        let black_tally = self.tally_pieces(black_pieces);
        println!("Black side: {:?}\n", black_tally);
    }
}

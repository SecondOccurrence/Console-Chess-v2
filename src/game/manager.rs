use crate::game::side::Side;
use crate::game::player::Player;
use crate::game::chess_board::ChessBoard;
use crate::game::menu::MenuFunctions;
use crate::game::pieces::{ Position, PieceType };

use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::fs::File;

pub struct GameManager {
    current_side: Side,   
    chess_board: ChessBoard,
    players: [Player; 2],
    move_again: bool,
    leave_game: bool,
}

impl GameManager {
    pub fn new() -> GameManager {
        let current_side = Side::WHITE;
        let chess_board = ChessBoard::new();
        let player1 = Player::new(Side::WHITE);
        let player2 = Player::new(Side::BLACK);
        let move_again = false;
        let leave_game = false;

        GameManager { 
            current_side, chess_board, 
            players: [player1, player2],
            move_again, leave_game
        }
    }

    pub fn run(&mut self) -> bool {
        if !self.move_again {
            self.display_board();
        }

        let side_index = self.current_side.to_index();

        if self.current_side == Side::WHITE {
            println!("\nWhite Move:");
        }
        else {
            println!("\nBlack Move:");
        }

        let player_move = self.players[side_index].move_input();
        if let Some(result_position) = player_move {
            self.players[side_index].generate_possible_moves();
            
            let player_pieces = self.players[side_index].pieces().clone();
            let opponent_pieces = self.players[side_index^1].pieces().clone();
            self.players[side_index].prune_possible_moves(player_pieces, false);
            self.players[side_index].prune_possible_moves(opponent_pieces, true);

            // TODO: side_index might not be correct. might be opposite
            if let Some(_piece) = self.players[side_index].get_piece(&result_position) {
                // TODO: print piece taken inform
            }

            let move_result = self.players[side_index].apply_move(&result_position);
            if let Ok(()) = move_result {
                self.move_again = false;

                self.chess_board.update_board(self.players[0].pieces(), self.players[1].pieces());
                self.current_side.switch();

                // clear the console screen through ANSI codes
                print!("{}[2J", 27 as char);
                print!("{}[1;1H", 27 as char);
            }
            else if let Err(err) = move_result {
                self.move_again = true;
                println!("{}, try again.", err);
            }
        }
        else if let None = player_move {
            self.move_again = false;
            // clear the console screen through ANSI codes
            print!("{}[2J", 27 as char);
            print!("{}[1;1H", 27 as char);

            self.enter_menu();

            print!("{}[2J", 27 as char);
            print!("{}[1;1H", 27 as char);
        }

        return self.leave_game;
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
    // TODO: add startup menu 
    fn enter_main_menu() {}
    // TODO: yerp
    fn main_menu_loop() {}

    fn enter_menu(&mut self) {
        println!("\n-- menu --");
        println!("enter \"exit\" to return to the game");
        println!("to show a list of available commands, enter \"help\"");
        self.menu_loop();
    }

    fn menu_loop(&mut self) {
        loop {
            println!("Enter an option:");
            let mut option = String::new();
            io::stdin().read_line(&mut option)
                .expect("Failed to read option");
            option = option.trim().to_string();

            self.perform_command(&option);
            if option == "exit"{
                break;
            }
            else if option == "close" {
                self.leave_game = true;
                break;
            }
        }
    }

    fn perform_command(&mut self, option: &str) {
        match option {
            "help" => self.help_menu(),
            "show" => self.display_board(),
            "pieces" => self.show_pieces_count(),
            "import" => self.import_game(),
            "export" => self.export_game(),
            "clear" => {
                // clear the console screen through ANSI codes
                print!("{}[2J", 27 as char);
                print!("{}[1;1H", 27 as char);
            }
            "exit" => println!("Exiting menu.."),
            "close" => self.begin_close(),
            _ => println!("'{}' is not a valid option", option),
        }
    }

    fn help_menu(&self) {
        println!("\n-- help menu --");
        println!("the following commands will do the following...\n");
        println!("\"help\"   => shows this");
        println!("\"show\"   => shows the current board state");
        println!("\"pieces\" => shows remaining pieces left on both sides");
        println!("\"import\" => creates a new game provided a save file using a FEN string");
        println!("\"export\" => saves the current game");
        println!("\"clear\"  => clears the screen to show a clean menu");
        println!("\"exit\"   => return to the game");
        println!("\"close\"  => close the chess game");
        println!("-- END --");
        // TODO: add new print for each available command
    }

    fn show_pieces_count(&self) {
        // TODO: format tally output better
        println!("\nRemaining Pieces:");
        let white_pieces = self.players[0].pieces();
        let white_tally = GameManager::tally_pieces(white_pieces);
        println!("White side: {:?}", white_tally);

        let black_pieces = self.players[1].pieces();
        let black_tally = GameManager::tally_pieces(black_pieces);
        println!("Black side: {:?}\n", black_tally);
    }

    fn tally_pieces(pieces: &HashMap<Position, PieceType>) -> HashMap<char, usize> {
        let mut counter: HashMap<char, usize> = HashMap::new();

        for (_, value) in pieces.iter() {
            let piece = value.icon();

            let count = counter.entry(piece).or_insert(0);
            *count += 1;
        }

        return counter;
    }

    // TODO: import game history?
    fn import_game(&mut self) {
        let save_states_path = GameManager::get_save_path();
        assert!(save_states_path.exists(), "Attempting to retrieve a save file: the path does not exist");

        let mut file_path: PathBuf;
        loop {
            file_path = GameManager::get_save_name(&save_states_path);
            if !file_path.exists() {
                println!("Save file does not exist.");
                continue;
            }

            break;
        }

        let file_contents = fs::read_to_string(file_path)
            .expect("Failed to read the save file.");

        // TODO: check if a pawn has its first move or not
        let validation = self.validate_save(&file_contents);
        if let Ok(()) = validation {
            self.read_save(&file_contents);
        }
        else if let Err(error) = validation {
            println!("Could not read the save file: {}", error);
        }

        self.chess_board.update_board(self.players[0].pieces(), self.players[1].pieces());
    }

    fn get_save_path() -> PathBuf {
        let path = dirs::home_dir()
            .expect("Failed to get your home directory");
        let path = path.join(".console-chess").join("saves");
        if !path.exists() {
            fs::create_dir_all(&path)
                .expect("Failed to create the save states folder");
        }

        return path;
    }

    fn get_save_name(path: &PathBuf) -> PathBuf {
        println!("Enter the save file name:");
        println!("(save files are located in: {})", path.display());
        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name)
            .expect("Failed to read save file");
        file_name = file_name.trim().to_string();
        file_name.push_str(".fen");

        let file_path = path.join(&file_name); 
        return file_path;
    }

    fn validate_save(&self, fen_string: &str) -> Result<(), String> {
        let input = if fen_string.ends_with('\n') {
            fen_string.trim_end()
        }
        else {
            &fen_string
        };

        let split_input: Vec<&str> = input.split('/').collect();
        if split_input.len() != 8 {
            return Err(format!("FEN string: expected 8 rows, found {}", split_input.len()));
        }


        for row in split_input.iter() {
            let mut cell_num = 0;
            for cell in row.chars() {
                if cell.is_digit(10) {
                    cell_num += cell.to_digit(10).unwrap();
                }
                else {
                    if let None = PieceType::convert(cell) {
                        return Err(format!("FEN string cell: expected piece, found '{}'", cell));
                    }
                    cell_num += 1;
                }
            }

            if cell_num != 8 {
                return Err(format!("FEN string row: expected 8 cells, found {}", cell_num));
            }
        }
        
        return Ok(());
    }

    fn read_save(&mut self, fen_string: &str) {
        self.chess_board.clear();
        self.players[0].clear_pieces();
        self.players[1].clear_pieces();

        let mut row = 7 as i8;
        let mut cell = 0 as i8;

        let input = if fen_string.ends_with('\n') {
            fen_string.trim_end()
        }
        else {
            &fen_string
        };

        let mut split_input = input.split('/');
        while let Some(substring) = split_input.next() {
            for ch in substring.chars() {
                if ch.is_digit(10) {
                    cell += ch.to_digit(10).unwrap() as i8; 
                }
                else {
                    let convert_result = PieceType::convert(ch);
                    assert!(convert_result.is_some(), "Piece conversion: expected some, but got none");
                    let piece = convert_result.unwrap();

                    let side = piece.side();
                    let pos = Position { x: cell, y: row };
                    // TODO: change first move if not at specific y-coord
                    if side == Side::WHITE {
                        self.players[0].add_piece(pos, piece);
                    }
                    else {
                        self.players[1].add_piece(pos, piece);
                    }
                    cell += 1;
                }
            }
            row -= 1;
            cell = 0;
        }
        println!("Finished loading save.\n-- END --\n");
    }

    fn export_game(&self) {
        let save_dir = GameManager::get_save_path();
        let mut save_path: PathBuf;
        loop {

            save_path = GameManager::get_save_name(&save_dir);

            let mut choice = String::new();
            if save_path.exists() {
                println!("Save file already exists. Do you wish to overwrite it?");
                io::stdin().read_line(&mut choice)
                    .expect("Failed to read save file");
                choice = choice.trim().to_string();
            }

            if choice != "n" && choice != "N" {
                break;
            }
        }

        let board_state = self.export_board();
        GameManager::create_save(&save_path, &board_state);
        println!("Finished creating save file.\n -- END --\n");
    }

    fn export_board(&self) -> String {
        let mut fen_string = String::new();
        for row in (0..8).rev() {
            let mut line = String::new();
            let mut empty = 0;
            for col in 0..8 {
                let cell = self.chess_board.get_piece(row, col);

                if cell == '.' {
                    empty += 1;
                }
                else {
                    if empty != 0 {
                        line.push_str(&empty.to_string());
                        empty = 0;
                    }
                    line.push(cell);
                }
            }

            if empty != 0 {
                line.push_str(&empty.to_string());
            }

            fen_string.push_str(&line);
            if row > 0 {
                fen_string.push('/');
            }
        }
        return fen_string;
    }

    fn create_save(save_path: &PathBuf, board_state: &str) {
        let mut save_file = File::create(save_path)
            .expect("Failed to create the save");
        save_file.write_all(board_state.as_bytes())
            .expect("Failed to write the save to the file");
    }

    fn begin_close(&self) {
        println!("Do you wish to save before closing? (Yy/Nn):");
        let mut choice = String::new();
        loop {
            io::stdin().read_line(&mut choice)
                .expect("Failed to read choice");
            choice = choice.trim().to_string();

            if choice != "Y" && choice != "y" && choice != "N" && choice != "n" {
                println!("Invalid choice. Type 'Y/y' or 'N/n'");
                choice = String::new();
            }
            else {
                break;
            }
        }

        if choice == "Y" || choice == "y" {
            self.export_game();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn board_length_upper_lim() {
        let gm = GameManager::new();
        let fen_string_1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8";
        assert_eq!(gm.validate_save(&fen_string_1), Ok(()));
    }

    #[test]
    #[should_panic]
    fn board_length_lower_lim() {
        let gm = GameManager::new();
        let fen_string_1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP";
        assert_eq!(gm.validate_save(&fen_string_1), Ok(()));
    }

    #[test]
    #[should_panic]
    fn board_row_length_1() {
        let gm = GameManager::new();
        let fen_string_1 = "rnbqkbnr/pppppppp/8/7/8/8/PPPPPPPP";
        assert_eq!(gm.validate_save(&fen_string_1), Ok(()));
    }

    #[test]
    #[should_panic]
    fn board_row_length_2() {
        let gm = GameManager::new();
        let fen_string_1 = "rnbqkbnrr/pppppppp/8/8/8/8/PPPPPPPP";
        assert_eq!(gm.validate_save(&fen_string_1), Ok(()));
    }

    #[test]
    #[should_panic]
    fn board_row_length_3() {
        let gm = GameManager::new();
        let fen_string_1 = "rnbqkbnrr/pppppppp/8/8/8/8/PPPPPPPP/";
        assert_eq!(gm.validate_save(&fen_string_1), Ok(()));
    }

    #[test]
    #[should_panic]
    fn board_cell_invalid_piece() {
        let gm = GameManager::new();
        let fen_string_1 = "rnbVkbnr/pppppppp/8/8/8/8/PPPPPPPP/";
        assert_eq!(gm.validate_save(&fen_string_1), Ok(()));
    }

    #[test]
    fn valid_board() {
        let gm = GameManager::new();

        let fen_string_1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let fen_string_2 = "r2qkbnr/p1p1pppp/b1np4/1p6/2B2P2/4P3/PPPP1KPP/RNBQ2NR";
        let fen_string_3 = "rNb1kN1r/ppp1pppp/3p1n2/8/Pq6/2PPP3/1P3PPP/RNB1KB1R";
        let fen_string_4 = "rNb1kN1r/ppp1ppRp/3p4/8/P6P/3qP3/5PP1/R3Kn2";

        assert_eq!(gm.validate_save(&fen_string_1), Ok(()));
        assert_eq!(gm.validate_save(&fen_string_2), Ok(()));
        assert_eq!(gm.validate_save(&fen_string_3), Ok(()));
        assert_eq!(gm.validate_save(&fen_string_4), Ok(()));
    }

    #[test]
    fn read_valid_board() {
        let mut gm = GameManager::new();
        let fen_string_1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        gm.validate_save(&fen_string_1).unwrap();
        gm.read_save(&fen_string_1);

        let p1_pieces = gm.players[0].pieces();
        let p2_pieces = gm.players[1].pieces();
        gm.chess_board.update_board(p1_pieces, p2_pieces);

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

        for row in 0..8 {
            for cell in 0..8 {
                let piece = gm.chess_board.get_piece(row, cell);
                assert_eq!(piece, board[row][cell], "Expected '{}', got '{}' at row:{}, cell:{}", piece, board[row][cell], row, cell);
            }
        }
    }

    #[test]
    fn test_board_export() {
        let gm = GameManager::new();

        let fen_string_export = gm.export_board();
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        assert_eq!(fen_string_export, fen_string, "Expected fen string: '{}', received: '{}'", fen_string, fen_string_export);
    }
}

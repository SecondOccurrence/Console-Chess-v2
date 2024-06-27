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
        let mut leave_game = false;

        self.display_board();

        let side_index = self.current_side.to_index();

        if self.current_side == Side::WHITE {
            println!("\nWhite Move:");
        }
        else {
            println!("\nBlack Move:");
        }

        let (initial_position, result_position) = self.players[side_index].move_input();
        // returns -1 if user wishes to enter the menu
        if initial_position.x != -1 {
            if self.players[side_index].piece_at_coord(&result_position) {
                // TODO: print piece taken inform
            }

            self.players[side_index].apply_move(&initial_position, &result_position);

            self.chess_board.update_board(self.players[0].pieces(), self.players[1].pieces());
            self.current_side.switch();
        }
        else {
            // clear the console screen through ANSI codes
            print!("{}[2J", 27 as char);
            print!("{}[1;1H", 27 as char);

            GameManager::show_menu();
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
                    leave_game = true;
                    break;
                }
            }
        }

        // clear the console screen through ANSI codes
        print!("{}[2J", 27 as char);
        print!("{}[1;1H", 27 as char);

        return leave_game;
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
    
    fn show_menu() {
        println!("\n-- menu --");
        println!("enter \"exit\" to return to the game");
        println!("to show a list of available commands, enter \"help\"");
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
                GameManager::show_menu();
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

    // TODO: import game history?
    fn import_game(&mut self) {
        let save_states_path = GameManager::get_save_path();
        let save_file_path = GameManager::retrieve_save_file(&save_states_path);

        let file_contents = fs::read_to_string(save_file_path)
            .expect("Failed to read the save file.");

        self.read_save(&file_contents);

        self.chess_board.update_board(self.players[0].pieces(), self.players[1].pieces());
    }

    fn retrieve_save_file(path: &PathBuf) -> PathBuf {
        assert!(path.exists(), "Attempting to retrieve a save file: the path does not exist");

        let mut file_path: PathBuf;
        loop {
            println!("Enter the save file name:");
            println!("(save files are located in: {})", path.display());
            let mut file_name = String::new();
            io::stdin().read_line(&mut file_name)
                .expect("Failed to read move");
            file_name = file_name.trim().to_string();
            file_name.push_str(".fen");

            file_path = path.join(file_name); 
            if let Err(_) = fs::File::open(&file_path) {
                println!("Save file does not exist.\n");
                continue;
            }
            
            break;
        }

        return file_path;
    }

    fn read_save(&mut self, fen_string: &str) {
        self.chess_board.clear();
        self.players[0].clear_pieces();
        self.players[1].clear_pieces();

        let mut row = 7 as i8;
        let mut cell = 0 as i8;

        let mut split_input = fen_string.split('/');
        while let Some(substring) = split_input.next() {
            for ch in substring.chars() {
                if ch.is_digit(10) {
                    cell += ch.to_digit(10).unwrap() as i8; 
                }
                else {
                    if let Some(piece) = PieceType::convert(ch) {
                        let side = piece.side();
                        let pos = Position { x: cell, y: row };
                        if side == Side::WHITE {
                            self.players[0].add_piece(pos, piece);
                        }
                        else {
                            self.players[1].add_piece(pos, piece);
                        }
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
        let board_state = self.export_board();
        GameManager::create_save(&board_state);
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

    fn create_save(board_state: &str) {
        let mut save_states_path = GameManager::get_save_path();

        println!("Enter the name to give the save:");
        println!("(save files are stored in: {})", save_states_path.display());

        loop {
            let mut save_name = String::new();
            io::stdin().read_line(&mut save_name)
                .expect("Failed to read the save name");
            save_name = save_name.trim().to_string();
            save_name.push_str(".fen");

            let temp_save_path = save_states_path.clone().join(save_name.clone());
            if !temp_save_path.exists() {
                save_states_path = save_states_path.join(save_name);
            }
            else {
                println!("Save already exists under that name");
                continue;
            }

            break;
        }

        let mut save_file = File::create(save_states_path)
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

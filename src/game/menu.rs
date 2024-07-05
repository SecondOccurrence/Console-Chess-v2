use crate::game::pieces::{ Position, PieceType };

use std::collections::HashMap;
use std::path::PathBuf;

pub trait MenuFunctions {
    fn enter_main_menu();
    fn main_menu_loop();

    fn enter_menu(&mut self);
    fn menu_loop(&mut self);
    fn perform_command(&mut self, option: &str);

    fn help_menu(&self);

    fn show_pieces_count(&self);
    fn tally_pieces(pieces: &HashMap<Position, PieceType>) -> HashMap<char, usize>;

    fn get_save_path() -> PathBuf;
    fn get_save_name(path: &PathBuf) -> PathBuf;

    fn import_game(&mut self);
    fn validate_save(&self, fen_string: &str) -> Result<(), String>;
    fn read_save(&mut self, fen_string: &str);

    fn export_game(&self);
    fn export_board(&self) -> String;
    fn create_save(save_path: &PathBuf, board_state: &str);

    fn begin_close(&self);
}

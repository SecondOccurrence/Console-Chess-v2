use crate::game::pieces::{ Position, PieceType };

use std::collections::HashMap;

pub trait MenuFunctions {
    fn show_menu();
    fn perform_command(&self, option: &str);

    fn help_menu(&self);

    fn show_pieces_count(&self);
    fn tally_pieces(pieces: &HashMap<Position, PieceType>) -> HashMap<char, usize>;

    fn import_game(&self);
}

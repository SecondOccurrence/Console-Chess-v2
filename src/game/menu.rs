use crate::game::pieces::{ Position, PieceType };

use std::collections::HashMap;

pub trait MenuFunctions {
    fn show_menu(&self);
    fn perform_command(&self, option: &str);
    fn help_menu(&self);
    fn show_pieces_count(&self);

    fn tally_pieces(&self, pieces: &HashMap<Position, PieceType>) -> HashMap<char, usize> {
        let mut counter: HashMap<char, usize> = HashMap::new();

        for (_, value) in pieces.iter() {
            let piece = value.icon();

            let count = counter.entry(piece).or_insert(0);
            *count += 1;
        }

        return counter;
    }
}

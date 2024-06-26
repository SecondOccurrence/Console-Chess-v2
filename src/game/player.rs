use crate::game::side::Side;
use crate::game::pieces::*;

use std::collections::HashMap;
use std::io;

pub struct Player {
    pieces: HashMap<Position, PieceType>,
}

impl Player {
    pub fn new(side: Side) -> Player {
        let pieces = Player::init_pieces(&side);
        Player { pieces }
    }

    fn init_pieces(side: &Side) -> HashMap<Position, PieceType> {

        let mut pos_y: i8;
        let increment: i8;

        if *side == Side::WHITE {
            pos_y = 0;
            increment = 1;
        }
        else {
            pos_y = 7;
            increment = -1;
        }

        let mut map = HashMap::new();

        map.insert(Position { x: 0, y: pos_y }, PieceType::Rook(Rook::new(*side)));
        map.insert(Position { x: 7, y: pos_y }, PieceType::Rook(Rook::new(*side)));
        map.insert(Position { x: 1, y: pos_y }, PieceType::Knight(Knight::new(*side)));
        map.insert(Position { x: 6, y: pos_y }, PieceType::Knight(Knight::new(*side)));
        map.insert(Position { x: 5, y: pos_y }, PieceType::Bishop(Bishop::new(*side)));
        map.insert(Position { x: 2, y: pos_y }, PieceType::Bishop(Bishop::new(*side)));
        map.insert(Position { x: 3, y: pos_y }, PieceType::Queen(Queen::new(*side)));
        map.insert(Position { x: 4, y: pos_y }, PieceType::King(King::new(*side)));

        pos_y = pos_y + increment;
        for pawn_x_pos in 0..8 {
            map.insert(Position { x: pawn_x_pos, y: pos_y}, PieceType::Pawn(Pawn::new(*side)));
        }

        return map;
    }

    pub fn move_input(&self) -> (Position, Position) {
        loop {
            let mut new_move = String::new();
            io::stdin().read_line(&mut new_move)
                .expect("Failed to read move");
            new_move = new_move.trim().to_string();

            let (mut initial_pos, mut result_pos) = (Position { x: -2, y: -2 }, Position { x: -2, y: -2 });
            if new_move != "menu" {
                let (temp_initial_pos, temp_result_pos) = self.validate_input(&new_move);
                initial_pos = temp_initial_pos;
                result_pos = temp_result_pos;
            }

            // x = -1 only exists when theres invalid input
            if initial_pos.x == -1 {
                println!("Invalid move. Try again.");
                continue;
            }

            return (initial_pos, result_pos);
        }
    }

    // TODO: refactor to contains key
    pub fn piece_at_coord(&self, pos: &Position) -> bool {
        let mut found = false;
        if let Some(_value) = self.pieces.get(pos) {
            found = true;
        }
        return found;
    }

    pub fn apply_move(&mut self, initial_pos: &Position, result_pos: &Position) {
        assert!(self.pieces.contains_key(initial_pos), "None of the players pieces are located at the initial position");

        let piece = self.pieces.remove(initial_pos).unwrap();

        self.pieces.insert(*result_pos, piece);
        println!("piece moved");
    }

    pub fn pieces(&self) -> &HashMap<Position, PieceType> {
        return &self.pieces;
    }

    pub fn clear_pieces(&mut self) {
        self.pieces.clear();
    }

    fn validate_input(&self, input: &str) -> (Position, Position) {
        let mut old_pos = Position { x: -1, y: -1 };
        let mut new_pos = Position { x: -1, y: -1 };

        if input.len() != 4 { 
            return (old_pos, new_pos);
        }

        let (initial_coord, result_coord) = input.split_at(2);

        if self.validate_coordinate(&initial_coord) && self.validate_coordinate(&result_coord) {
            let old_pos_x_letter = initial_coord.chars().nth(0).unwrap();
            let new_pos_x_letter = result_coord.chars().nth(0).unwrap();

            let old_pos_x = (old_pos_x_letter as u8 - b'a') as i8;
            let new_pos_x = (new_pos_x_letter as u8 - b'a') as i8;


            let old_pos_y = initial_coord.chars().nth(1).unwrap().to_digit(10).unwrap() as i8 - 1;
            let new_pos_y = result_coord.chars().nth(1).unwrap().to_digit(10).unwrap() as i8 - 1;

            let temp_old_pos = Position { x: old_pos_x, y: old_pos_y };
            let temp_new_pos = Position { x: new_pos_x, y: new_pos_y };

            if self.piece_at_coord(&temp_old_pos) && !self.piece_at_coord(&temp_new_pos) {
                old_pos = temp_old_pos;
                new_pos = temp_new_pos;
            }
            else {
                println!("Move must be performed on your piece \nMove must not land on another piece of yours.");
            }
        }
        else {
            println!("Be sure the move lies within the board coordinates");
        }

        return (old_pos, new_pos);
    }

    fn validate_coordinate(&self, coord: &str) -> bool {
        assert!(coord.len() == 2, "Coordinate string must have exactly two characters.");

        let pos_x = coord.chars().nth(0).unwrap();
        let pos_y = coord.chars().nth(1).unwrap();

        if pos_x >= 'a' && pos_x <= 'h' && pos_y >= '1' && pos_y <= '8' {
            return true;
        }
        else {
            return false;
        }
    }
}

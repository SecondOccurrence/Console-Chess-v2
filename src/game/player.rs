use crate::game::side::Side;
use crate::game::pieces::*;
use crate::game::move_direction::MoveDirection;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

pub struct Player {
    pieces: HashMap<Position, PieceType>,
    current_piece: (Position, PieceType),
    possible_moves: HashSet<Position>,
}

impl Player {
    pub fn new(side: Side) -> Player {
        let pieces = Player::init_pieces(&side);
        let current_piece = ( Position { x: -1, y: -1 }, PieceType::Pawn(Pawn::new(side)) );
        let possible_moves = HashSet::new();
        Player { pieces, current_piece, possible_moves }
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

    pub fn move_input(&mut self) -> Option<(Position, Position)> {
        let (initial_pos, result_pos): (Position, Position);
        loop {
            let mut new_move = String::new();
            io::stdin().read_line(&mut new_move)
                .expect("Failed to read move");
            new_move = new_move.trim().to_string();

            if new_move == "menu" {
                return None;
            }

            let validation = self.validate_input(&new_move);
            if let Err(error) = validation {
                println!("{}", error);
            }
            else if let Ok((initial, result)) = validation {
                initial_pos = initial;
                result_pos = result;
                break;
            }
        }

        return Some((initial_pos, result_pos));
    }

    fn validate_input(&mut self, input: &str) -> Result<(Position, Position), String> {
        if input.len() != 4 { 
            return Err("Move must follow the example format: a1a2".to_string());
        }

        let (initial_coord, result_coord) = input.split_at(2);

        if !self.validate_coordinate(&initial_coord) || !self.validate_coordinate(&result_coord) {
            return Err("Be sure the move lies within the board coordinates".to_string());
        }

        let old_pos_x_letter = initial_coord.chars().nth(0).unwrap();
        let new_pos_x_letter = result_coord.chars().nth(0).unwrap();

        let old_pos_x = (old_pos_x_letter as u8 - b'a') as i8;
        let new_pos_x = (new_pos_x_letter as u8 - b'a') as i8;

        let old_pos_y = initial_coord.chars().nth(1).unwrap().to_digit(10).unwrap() as i8 - 1;
        let new_pos_y = result_coord.chars().nth(1).unwrap().to_digit(10).unwrap() as i8 - 1;

        let old_pos = Position { x: old_pos_x, y: old_pos_y };
        let new_pos = Position { x: new_pos_x, y: new_pos_y };

        if let None = self.get_piece(&old_pos) {
            return Err("Move must be performed on your piece".to_string());
        }

        if let Some(_) = self.get_piece(&new_pos) {
            return Err("Move must not land on another piece of yours".to_string());
        }

        return Ok((old_pos, new_pos));
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

    pub fn apply_move(&mut self, initial_pos: &Position, result_pos: &Position) {
        assert!(self.pieces.contains_key(initial_pos), "None of the players pieces are located at the initial position");

        let piece = self.pieces.remove(initial_pos).unwrap();

        self.pieces.insert(*result_pos, piece);
        println!("piece moved");
    }

    pub fn pieces(&self) -> &HashMap<Position, PieceType> {
        return &self.pieces;
    }

    pub fn get_piece(&self, pos: &Position) -> Option<&PieceType> {
        return self.pieces.get(&pos);
    }

    pub fn get_piece_mut(&mut self, pos: &Position) -> Option<&mut PieceType> {
        return self.pieces.get_mut(&pos);
    }

    pub fn add_piece(&mut self, pos: Position, piece: PieceType) {
        assert!(pos.x < 8 && pos.y < 8, "Adding piece at position ({},{}) that lies off the board", pos.x, pos.y);
        assert!(!self.pieces.contains_key(&pos), "Adding a piece at a position ({},{}) which already contains a piece", pos.x, pos.y);

        self.pieces.insert(pos, piece);
    }

    pub fn clear_pieces(&mut self) {
        self.pieces.clear();
    }

    pub fn generate_possible_moves(&mut self, piece_at_pos: &Position) {
        assert!(self.pieces.contains_key(&piece_at_pos), "Generating moves for a piece that does not exist at ({},{})", piece_at_pos.x, piece_at_pos.y);

        self.current_piece = (*piece_at_pos, self.pieces.get(&piece_at_pos).unwrap().clone());
        self.possible_moves = self.pieces.get_mut(&piece_at_pos).unwrap().possible_moves(&piece_at_pos);
    }

    pub fn prune_possible_moves(&mut self, pieces_to_compare: HashMap<Position, PieceType>, capturing: bool) {
        assert!(self.current_piece.0.x != -1, "Pruning possible moves when no current piece is known to the player.");

        for (itr_pos, itr_piece) in pieces_to_compare.iter() {
            if let Some(found_pos) = self.possible_moves.get(itr_pos) {
                let x_diff = (self.current_piece.0.x - found_pos.x) as u8;
                let y_diff = (self.current_piece.0.y - found_pos.y) as u8;

                let moves_to_prune = self.current_piece.1.invalid_moves(&self.possible_moves, &self.current_piece.0, x_diff, y_diff);

                // TODO: find direction from initial, x, y
                // lies on move path, prune beyond it
            }
        }
        // TODO: ^ use piece specific pruning
        //
        // TODO: ^ for bishop: generate positions from that point in that direction onwards

    }
}

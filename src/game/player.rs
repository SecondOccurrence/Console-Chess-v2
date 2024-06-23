use crate::game::side::Side;
use crate::game::pieces::*;
use std::collections::HashMap;
use std::io;

pub struct Player {
    side: Side,
    pieces: HashMap<Position, PieceType>,
    
}

impl Player {
    pub fn new(side: Side) -> Player {
        let pieces = Player::init_pieces(&side);
        Player { side, pieces }
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
        map.insert(Position { x: 4, y: pos_y }, PieceType::Bishop(Bishop::new(*side)));
        map.insert(Position { x: 3, y: pos_y }, PieceType::Queen(Queen::new(*side)));
        map.insert(Position { x: 2, y: pos_y }, PieceType::King(King::new(*side)));

        pos_y = pos_y + increment;
        for pawn_x_pos in 0..8 {
            map.insert(Position { x: pawn_x_pos, y: pos_y}, PieceType::Pawn(Pawn::new(*side)));
        }

        return map;
    }

    // TODO: Assign pieces to the player at different coordinates
    // TODO: Get input -> Validate input -> Move Piece

    pub fn move_input(&self) -> String {
        let mut new_move = String::new();
        let mut valid = false;

        while valid == false {
            io::stdin().read_line(&mut new_move).expect("uhh");
            new_move = new_move.trim().to_string();
            valid = self.validate_input(&new_move);

            if valid == false {
                println!("Invalid move. Try again.");
                new_move = String::new();
            }
        }

        return new_move;
    }

    fn validate_input(&self, input: &str) -> bool {
        if input.len() != 4 { 
            return false;
        }

        let mut valid = true;
        let (initial_coord, result_coord) = input.split_at(2);

        let initial_valid = self.validate_coordinate(&initial_coord);
        let result_valid = self.validate_coordinate(&result_coord);

        if !(initial_valid && result_valid) {
            valid = false;
        }

        return valid;
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

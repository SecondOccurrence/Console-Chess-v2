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

    pub fn move_input(&mut self) -> Option<Position> {
        let result_pos: Position;
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
                self.current_piece.0 = initial;
                self.current_piece.1 = self.get_piece(&initial).unwrap().clone();

                result_pos = result;
                break;
            }
        }

        return Some(result_pos);
    }

    fn validate_input(&self, input: &str) -> Result<(Position, Position), String> {
        if input.len() != 4 { 
            return Err("Move must follow the example format: a1a2".to_string());
        }

        let (initial_coord, result_coord) = input.split_at(2);

        if !Player::validate_coordinate(&initial_coord) || !Player::validate_coordinate(&result_coord) {
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

    fn validate_coordinate(coord: &str) -> bool {
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

    pub fn apply_move(&mut self, result_pos: &Position) {
        assert!(self.pieces.contains_key(&self.current_piece.0), "None of the players pieces are located at the initial position");

        let piece = self.pieces.remove(&self.current_piece.0).unwrap();

        self.pieces.insert(*result_pos, piece);
        println!("piece moved");
    }

    pub fn pieces(&self) -> &HashMap<Position, PieceType> {
        return &self.pieces;
    }

    pub fn get_piece(&self, pos: &Position) -> Option<&PieceType> {
        return self.pieces.get(&pos);
    }

    pub fn add_piece(&mut self, pos: Position, piece: PieceType) {
        assert!(pos.x < 8 && pos.y < 8, "Adding piece at position ({},{}) that lies off the board", pos.x, pos.y);
        assert!(!self.pieces.contains_key(&pos), "Adding a piece at a position ({},{}) which already contains a piece", pos.x, pos.y);

        self.pieces.insert(pos, piece);
    }

    pub fn clear_pieces(&mut self) {
        self.pieces.clear();
    }

    // TODO: test
    pub fn generate_possible_moves(&mut self) {
        let piece_at_pos = self.current_piece.0;

        assert!(self.pieces.contains_key(&piece_at_pos), "Generating moves for a piece that does not exist at ({},{})", piece_at_pos.x, piece_at_pos.y);

        self.current_piece = (piece_at_pos, self.pieces.get(&piece_at_pos).unwrap().clone());

        let up_left_moves = self.current_piece.1.generate_moves(&piece_at_pos, MoveDirection::UpLeft);
        let up_right_moves = self.current_piece.1.generate_moves(&piece_at_pos, MoveDirection::UpRight);
        let down_left_moves = self.current_piece.1.generate_moves(&piece_at_pos, MoveDirection::DownLeft);
        let down_right_moves = self.current_piece.1.generate_moves(&piece_at_pos, MoveDirection::DownRight);

        self.possible_moves = up_left_moves
            .union(&up_right_moves)
            .cloned()
            .chain(down_left_moves.union(&down_right_moves).cloned())
            .collect();
    }

    // TODO: test
    pub fn prune_possible_moves(&mut self, pieces_to_compare: HashMap<Position, PieceType>, capturing: bool) {
        assert!(self.current_piece.0.x == -1, "Pruning possible moves when no current piece is known to the player.");

        for (itr_pos, _) in pieces_to_compare.iter() {
            let piece_in_path = match self.possible_moves.get(itr_pos) {
                Some(found_pos) => found_pos,
                None => continue,
            };

            let found_pos = *piece_in_path;

            let x_diff = (self.current_piece.0.x - found_pos.x) as i8;
            let y_diff = (self.current_piece.0.y - found_pos.y) as i8;

            let mut moves_to_prune = self.current_piece.1.invalid_moves(&self.current_piece.0, x_diff, y_diff);
            if !capturing {
                moves_to_prune.insert(found_pos);
            }

            self.prune_moves(&moves_to_prune);
        }
    }

    // TODO: test
    fn prune_moves(&mut self, moves_to_prune: &HashSet<Position>) { 
        for pruned_move in moves_to_prune {
            if self.possible_moves.contains(&pruned_move) {
                self.possible_moves.remove(&pruned_move);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_valid() {
        assert!(Player::validate_coordinate("a1"), "Expected 'a1' to be a valid coordinate");
        assert!(Player::validate_coordinate("d3"), "Expected 'd3' to be a valid coordinate");
        assert!(Player::validate_coordinate("h8"), "Expected 'h7' to be a valid coordinate");
    }

    #[test]
    fn coord_invalid_outside() {
        assert!(!Player::validate_coordinate("a9"), "Expected 'a9' to be invalid coordinate");
        assert!(!Player::validate_coordinate("j1"), "Expected 'j1' to be invalid coordinate");
    }

    #[test]
    #[should_panic]
    fn coord_invalid_invalid() {
        assert!(Player::validate_coordinate("ddddd"), "Expected 'dddddd' to not be a coordinate");
    }

    #[test]
    fn valid_input() {
        let p = Player::new(Side::WHITE);

        assert!(p.validate_input("d2d3").is_ok(), "Expected 'd2d3' to be a valid move");
        assert!(p.validate_input("a1a4").is_ok(), "Expected 'a1a4' to be a valid move");
        assert!(p.validate_input("h1h4").is_ok(), "Expected 'h1h4' to be a valid move");
    }

    #[test]
    fn invalid_move() {
        let p = Player::new(Side::WHITE);

        assert!(p.validate_input("d3d4").is_err(), "Expected 'd3d4' to be invalid as not on player piece");
        assert!(p.validate_input("a1a2").is_err(), "Expected 'd3d4' to be invalid as moving on your piece");
        assert!(p.validate_input("movingpiece").is_err(), "Expected 'movingpiece' to be invalid as it isnt a move");
    }

    #[test]
    fn apply_move() {
        let mut p = Player::new(Side::WHITE);

        p.current_piece.0 = Position { x: 3, y: 1 };
        assert!(p.get_piece(&p.current_piece.0).is_some(), "Expected piece at position ({},{}) to exist before moving", p.current_piece.0.x, p.current_piece.0.y);

        p.current_piece.1 = p.get_piece(&p.current_piece.0).unwrap().clone();

        let move_to = Position { x: 3, y: 2 };

        p.apply_move(&move_to);
        assert!(p.get_piece(&move_to).is_some(), "Expected piece at position ({},{}) to exist", move_to.x, move_to.y);
        assert!(p.get_piece(&p.current_piece.0).is_none(), "Expected piece at position ({},{}) to not exist", p.current_piece.0.x, p.current_piece.0.y);
    }

    #[test]
    #[should_panic]
    fn apply_move_on_nothing() {
        let mut p = Player::new(Side::WHITE);

        p.current_piece.0 = Position { x: 3, y: 3 };
        assert!(p.get_piece(&p.current_piece.0).is_some(), "Expected piece at position ({},{}) to exist before moving", p.current_piece.0.x, p.current_piece.0.y);

        let move_to = Position { x: 3, y: 2 };

        p.apply_move(&move_to);
    }

    #[test]
    fn add_new_piece() {
        let mut p = Player::new(Side::WHITE);

        let pos = Position { x: 4, y: 4 };
        p.add_piece(pos, PieceType::Pawn(Pawn::new(Side::WHITE)));
    }

    #[test]
    #[should_panic]
    fn add_piece_that_exists() {
        let mut p = Player::new(Side::WHITE);

        let pos = Position { x: 1, y: 1 };
        p.add_piece(pos, PieceType::Pawn(Pawn::new(Side::WHITE)));
    }

    #[test]
    #[should_panic]
    fn add_piece_out_of_bounds() {
        let mut p = Player::new(Side::WHITE);

        let pos = Position { x: 9, y: 1 };
        p.add_piece(pos, PieceType::Pawn(Pawn::new(Side::WHITE)));
    }
}


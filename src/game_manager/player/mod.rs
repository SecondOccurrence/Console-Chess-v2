use std::io;

pub struct Player {
    // current pieces map key coord value piece char
}

impl Player {
    pub fn new() -> Player {
        Player {}
    }

    pub fn move_input(&self) -> String {
        let mut new_move = String::new();

        println!("White move:");
        io::stdin().read_line(&mut new_move).expect("uhh");
        new_move = new_move.trim().to_string();

        let valid = self.validate_input(&new_move);

        println!("Move is {}", valid);

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

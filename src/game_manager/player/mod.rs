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

        _ = self.validate_input(&new_move);

        return new_move;
    }

    fn validate_input(&self, input: &str) -> bool {
        if input.len() != 4 { 
            return false;
        }

        let mut valid = true;
        let (initial_coord, result_coord) = input.split_at(2);

        for index in 0..4 {
            println!("{:?} ", input.chars().nth(index));
            // TODO: iterate through input maybe split into coords validate char, number
        }


        println!("{}, {}", initial_coord, result_coord);


        return valid;
    }
}

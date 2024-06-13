mod game_manger;

use game_manger::{Side, GameManager};

fn main() {
    let mut gm = GameManager::new(Side::WHITE);

    let mut quit = false;

    while !quit {
        quit = gm.run();
    }

    println!("Game Ended.");
}

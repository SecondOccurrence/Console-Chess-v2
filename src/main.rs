mod game_manager;

use game_manager::{ Side, GameManager };

fn main() {
    let mut gm = GameManager::new(Side::WHITE);

    let mut quit = false;

    while !quit {
        quit = gm.run();
    }

    println!("Game Ended.");
}

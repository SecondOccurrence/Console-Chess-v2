mod game;

use game::manager::GameManager;

fn main() {
    let mut gm = GameManager::new();

    let mut quit = false;

    // TODO: show a main menu here before entering game
    while !quit {
        print!("{}[2J", 27 as char);
        print!("{}[1;1H", 27 as char);
        quit = gm.run();
    }

    println!("Game Ended.");
}

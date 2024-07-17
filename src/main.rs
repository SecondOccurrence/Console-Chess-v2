mod game;

use game::manager::GameManager;
use game::menu::MenuFunctions;

fn main() {
    let mut gm = GameManager::new();

    let mut quit = false;

    print!("{}[2J", 27 as char);
    print!("{}[1;1H", 27 as char);
    gm.enter_main_menu();
    while !quit {
        quit = gm.run();
    }

    println!("Game Ended.");
}

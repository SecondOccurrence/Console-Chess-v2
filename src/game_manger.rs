#[derive(PartialEq)]
pub enum Side {
    WHITE,
    BLACK,
}

impl Side {
    fn switch(&mut self) {
        *self = match *self {
            Side::WHITE => Side::BLACK,
            Side::BLACK => Side::WHITE,
        };
    }
}

pub struct GameManager {
    side: Side,   
}

impl GameManager {
    pub fn new(side: Side) -> GameManager {
        GameManager { side }
    }

    pub fn run(&mut self) -> bool {
        if self.side == Side::WHITE {
            println!("White turn.");
        }
        else {
            println!("Black turn.");
        }

        self.side.switch();

        true
    }
}

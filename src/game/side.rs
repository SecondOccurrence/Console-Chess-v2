#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    WHITE = 0,
    BLACK = 1,
}

impl Side {
    pub fn switch(&mut self) {
        *self = match *self {
            Side::WHITE => Side::BLACK,
            Side::BLACK => Side::WHITE,
        };
    }

    pub fn to_index(&self) -> usize {
        match self {
            Side::WHITE => 0,
            Side::BLACK => 1,
        }
    }
}


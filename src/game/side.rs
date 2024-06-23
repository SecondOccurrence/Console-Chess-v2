#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    WHITE,
    BLACK,
}

impl Side {
    pub fn switch(&mut self) {
        *self = match *self {
            Side::WHITE => Side::BLACK,
            Side::BLACK => Side::WHITE,
        };
    }
}

const DIGIT_WIDTH : usize = 3;
const DIGIT_HEIGHT : usize = 3;
const ACCOUNT_LENGTH: usize = 9;
const LINE_LENGTH : usize = DIGIT_WIDTH * ACCOUNT_LENGTH;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Blank,
    Pipe,
    Underscore,
}

#[derive(Clone, Copy)]
pub struct Digit {
    data: [Cell; 9],
}

pub fn new_empty_digit() -> Digit {
    Digit {
        data: [Cell::Blank; 9],
    }
}

impl Digit {
    pub fn at(&self, row: usize, col: usize) -> Cell {
        return self.data[row * DIGIT_WIDTH + col];
    }

    pub fn set_at(&mut self, row: usize, col: usize, val: Cell) {
        self.data[row * DIGIT_WIDTH + col] = val;
    }
}

pub fn read_digits(input: &[&str; 4]) -> [Digit; 9] {
    let mut result = [new_empty_digit(); 9];

    return result;
}


#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Blank,
    Pipe,
    Underscore,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Digit {
    data: [Cell; 9],
}

pub fn new_empty_digit() -> Digit {
    Digit {
        data: [Cell::Blank; 9],
    }
}

pub const ZERO : Digit = Digit {
    data: [Cell::Blank, Cell::Underscore, Cell::Blank, 
           Cell::Pipe, Cell::Blank, Cell::Pipe, 
           Cell::Pipe, Cell::Underscore, Cell::Pipe]
};

pub const ONE : Digit = Digit {
    data: [Cell::Blank,Cell::Blank,Cell::Blank,
           Cell::Blank,Cell::Blank,Cell::Pipe,
           Cell::Blank,Cell::Blank,Cell::Pipe]
};

pub const TWO : Digit = Digit {
    data: [Cell::Blank,Cell::Underscore,Cell::Blank,
           Cell::Blank,Cell::Underscore,Cell::Pipe,
           Cell::Pipe,Cell::Underscore,Cell::Blank]
};

pub const THREE : Digit = Digit {
    data: [Cell::Blank, Cell::Underscore, Cell::Blank,
           Cell::Blank, Cell::Underscore, Cell::Pipe,
           Cell::Blank, Cell::Underscore, Cell::Pipe,]
};

pub const FOUR : Digit = Digit {
    data: [Cell::Blank, Cell::Blank, Cell::Blank,
           Cell::Pipe, Cell::Underscore, Cell::Pipe,
           Cell::Blank, Cell::Blank, Cell::Pipe,]
};

pub const FIVE : Digit = Digit {
    data: [Cell::Blank, Cell::Underscore, Cell::Blank,
           Cell::Pipe, Cell::Underscore, Cell::Blank,
           Cell::Blank, Cell::Underscore, Cell::Pipe,]
};

pub const SIX : Digit = Digit {
    data: [Cell::Blank, Cell::Underscore, Cell::Blank,
           Cell::Pipe, Cell::Underscore, Cell::Blank,
           Cell::Pipe, Cell::Underscore, Cell::Pipe,]
};

pub const SEVEN : Digit = Digit {
    data: [Cell::Blank, Cell::Underscore, Cell::Blank,
           Cell::Blank, Cell::Blank, Cell::Pipe,
           Cell::Blank, Cell::Blank, Cell::Pipe,]
};

pub const EIGHT : Digit = Digit {
    data: [Cell::Blank, Cell::Underscore, Cell::Blank,
           Cell::Pipe, Cell::Underscore, Cell::Pipe,
           Cell::Pipe, Cell::Underscore, Cell::Pipe,]
};

pub const NINE : Digit = Digit {
    data: [Cell::Blank, Cell::Underscore, Cell::Blank,
           Cell::Pipe, Cell::Underscore, Cell::Pipe,
           Cell::Blank, Cell::Underscore, Cell::Pipe,]
};

const DIGITS : [Digit; 10] = [
    ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE
];

pub const ACCOUNT_LENGTH: usize = 9;
pub const DIGIT_WIDTH : usize = 3;
const DIGIT_HEIGHT : usize = 3;

impl Digit {

    pub fn at(&self, row: usize, col: usize) -> Cell {
        return self.data[row * DIGIT_WIDTH + col];
    }

    pub fn set_at(&mut self, row: usize, col: usize, val: Cell) {
        self.data[row * DIGIT_WIDTH + col] = val;
    }

    pub fn value(&self) -> Option<u32> {
        for index in 0..DIGITS.len() {
            if DIGITS[index] == *self {
                return Some(index as u32);
            }
        }
        return None;
    }
}

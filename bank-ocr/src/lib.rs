const DIGIT_WIDTH : usize = 3;
const DIGIT_HEIGHT : usize = 3;
pub const ACCOUNT_LENGTH: usize = 9;

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

pub fn interpret_digits(entry: [Digit; ACCOUNT_LENGTH]) -> Option<u32> {
    const BASE : u32 = 10;
    let mut power : u32 = 1;
    let mut account : u32 = 0;
    for index in (0..ACCOUNT_LENGTH).rev() {
        if let Some(value) = entry[index].value() {
            account +=  value * power;
            power *= BASE;
        }
        else {
            return None;
        }
    }
    return Some(account);
}

pub fn read_digits(input: &[&str; 4]) -> [Digit; ACCOUNT_LENGTH] {
    let mut result = [new_empty_digit(); ACCOUNT_LENGTH];

    for row in 0..3 {
        let line = input[row];
        for (col_in_row, character) in line.char_indices() {
            let digit_index = col_in_row / DIGIT_WIDTH;
            let col = col_in_row % DIGIT_WIDTH;
            let cell = match character {
                ' ' => Cell::Blank,
                '_' => Cell::Underscore,
                '|' => Cell::Pipe,
                other => panic!("unknown character {}", other),
            };
            result[digit_index].set_at(row, col, cell);
        }
    }

    return result;
}

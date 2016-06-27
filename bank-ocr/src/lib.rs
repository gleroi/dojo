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


impl Digit {
    pub fn at(&self, row: usize, col: usize) -> Cell {
        return self.data[row * DIGIT_WIDTH + col];
    }

    pub fn set_at(&mut self, row: usize, col: usize, val: Cell) {
        self.data[row * DIGIT_WIDTH + col] = val;
    }
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

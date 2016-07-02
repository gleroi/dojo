use std::ops::{Index, IndexMut};
use reader::FileEntry;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Blank,
    Pipe,
    Underscore,
}

/// `Digit` from an account number.
///
/// A `Digit` is a square of 9 `Cell`s.
/// Certains patterns of cells represent valid numbers, between 0 and 9.
#[derive(Clone, Copy, PartialEq)]
pub struct Digit {
    data: [Cell; 9],
}

pub const ZERO: Digit = Digit {
    data: [Cell::Blank,
           Cell::Underscore,
           Cell::Blank,
           Cell::Pipe,
           Cell::Blank,
           Cell::Pipe,
           Cell::Pipe,
           Cell::Underscore,
           Cell::Pipe],
};

pub const ONE: Digit = Digit {
    data: [Cell::Blank,
           Cell::Blank,
           Cell::Blank,
           Cell::Blank,
           Cell::Blank,
           Cell::Pipe,
           Cell::Blank,
           Cell::Blank,
           Cell::Pipe],
};

pub const TWO: Digit = Digit {
    data: [Cell::Blank,
           Cell::Underscore,
           Cell::Blank,
           Cell::Blank,
           Cell::Underscore,
           Cell::Pipe,
           Cell::Pipe,
           Cell::Underscore,
           Cell::Blank],
};

pub const THREE: Digit = Digit {
    data: [Cell::Blank,
           Cell::Underscore,
           Cell::Blank,
           Cell::Blank,
           Cell::Underscore,
           Cell::Pipe,
           Cell::Blank,
           Cell::Underscore,
           Cell::Pipe],
};

pub const FOUR: Digit = Digit {
    data: [Cell::Blank,
           Cell::Blank,
           Cell::Blank,
           Cell::Pipe,
           Cell::Underscore,
           Cell::Pipe,
           Cell::Blank,
           Cell::Blank,
           Cell::Pipe],
};

pub const FIVE: Digit = Digit {
    data: [Cell::Blank,
           Cell::Underscore,
           Cell::Blank,
           Cell::Pipe,
           Cell::Underscore,
           Cell::Blank,
           Cell::Blank,
           Cell::Underscore,
           Cell::Pipe],
};

pub const SIX: Digit = Digit {
    data: [Cell::Blank,
           Cell::Underscore,
           Cell::Blank,
           Cell::Pipe,
           Cell::Underscore,
           Cell::Blank,
           Cell::Pipe,
           Cell::Underscore,
           Cell::Pipe],
};

pub const SEVEN: Digit = Digit {
    data: [Cell::Blank,
           Cell::Underscore,
           Cell::Blank,
           Cell::Blank,
           Cell::Blank,
           Cell::Pipe,
           Cell::Blank,
           Cell::Blank,
           Cell::Pipe],
};

pub const EIGHT: Digit = Digit {
    data: [Cell::Blank,
           Cell::Underscore,
           Cell::Blank,
           Cell::Pipe,
           Cell::Underscore,
           Cell::Pipe,
           Cell::Pipe,
           Cell::Underscore,
           Cell::Pipe],
};

pub const NINE: Digit = Digit {
    data: [Cell::Blank,
           Cell::Underscore,
           Cell::Blank,
           Cell::Pipe,
           Cell::Underscore,
           Cell::Pipe,
           Cell::Blank,
           Cell::Underscore,
           Cell::Pipe],
};

const DIGITS: [Digit; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

pub const ACCOUNT_LENGTH: usize = 9;
pub const DIGIT_WIDTH: usize = 3;

impl Digit {
    /// Create an empty Digit
    ///
    /// An empty digit is a `Digit` which cells are all `Cell::Blank`
    pub fn empty() -> Digit {
        Digit { data: [Cell::Blank; 9] }
    }

    pub fn at(&self, row: usize, col: usize) -> Cell {
        return self.data[row * DIGIT_WIDTH + col];
    }

    pub fn set_at(&mut self, row: usize, col: usize, val: Cell) {
        self.data[row * DIGIT_WIDTH + col] = val;
    }

    /// retuns the value between [0-9] of a Digit, or `None`
    pub fn value(&self) -> Option<u32> {
        for index in 0..DIGITS.len() {
            if DIGITS[index] == *self {
                return Some(index as u32);
            }
        }
        return None;
    }
}

/// An account number, composed of nine `Digits`
///
/// A `Account` can be created by using the function `parse(&FileEntry)`
pub struct Account {
    data: [Digit; ACCOUNT_LENGTH],
}

impl Account {
    fn new() -> Account {
        return Account { data: [Digit::empty(); ACCOUNT_LENGTH] };
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn value(&self) -> Option<u32> {
        const BASE: u32 = 10;
        let mut power: u32 = 1;
        let mut account: u32 = 0;
        for index in (0..ACCOUNT_LENGTH).rev() {
            if let Some(value) = self[index].value() {
                account += value * power;
                power *= BASE;
            } else {
                return None;
            }
        }
        return Some(account);
    }
    pub fn description(&self) -> String {
        let mut result = String::with_capacity(ACCOUNT_LENGTH);
        for digit in &self.data {
            if let Some(value) = digit.value() {
                result.push_str(&value.to_string());
            }
            else {
                result.push('?');
            }
        }
        return result;
    }
}

impl Index<usize> for Account {
    type Output = Digit;

    fn index(&self, index: usize) -> &Digit {
        return &self.data[index];
    }
}

impl IndexMut<usize> for Account {
    fn index_mut(&mut self, index: usize) -> &mut Digit {
        return &mut self.data[index];
    }
}

pub fn parse(input: &FileEntry) -> Account {
    let mut result = Account::new();

    for row in 0..3 {
        let line = &input[row];
        assert!(line.len() == 27);
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
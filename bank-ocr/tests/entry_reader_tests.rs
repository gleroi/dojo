extern crate bank_ocr;

use bank_ocr::*;

const ZERO : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _",
"| || || || || || || || || |",
"|_||_||_||_||_||_||_||_||_|",
"                           ",
];

#[test]
fn empty_digit_should_contains_blank_only() {
    let empty = new_empty_digit();

    for row in 0..3 {
        for col in 0..3 {
            assert!(empty.at(row, col) == Cell::Blank);
        }
    }
}

#[test]
fn read_digits_of_zeros_should_returns_nine_zeros() {
    let zero = read_digits(&ZERO);

    assert!(zero.len() == 9);
}

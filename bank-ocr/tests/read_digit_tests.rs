extern crate bank_ocr;

use bank_ocr::*;


#[test]
fn empty_digit_should_contains_blank_only() {
    let empty = new_empty_digit();

    for row in 0..3 {
        for col in 0..3 {
            assert!(empty.at(row, col) == Cell::Blank);
        }
    }
}

const TWO_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ", 
" _| _| _| _| _| _| _| _| _|",
"|_ |_ |_ |_ |_ |_ |_ |_ |_ ",
"                           ",
];

#[test]
fn read_digits_of_twos_should_returns_nine_twos() {
    let zero = read_digits(&TWO_INPUT);

    assert!(zero.len() == ACCOUNT_LENGTH);
    for index in 0..ACCOUNT_LENGTH {
        assert!(zero[index] == TWO);
    }
}


const ONE_INPUT : [&'static str; 4] = [
"                           ",
"  |  |  |  |  |  |  |  |  |",
"  |  |  |  |  |  |  |  |  |",
"                           ",
];

#[test]
fn read_digits_of_ones_should_returns_nine_ones() {
    let zero = read_digits(&ONE_INPUT);

    assert!(zero.len() == ACCOUNT_LENGTH);
    for index in 0..ACCOUNT_LENGTH {
        assert!(zero[index] == ONE);
    }
}


const ZERO_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _",
"| || || || || || || || || |",
"|_||_||_||_||_||_||_||_||_|",
"                           ",
];

#[test]
fn read_digits_of_zeros_should_returns_nine_zeros() {
    let zero = read_digits(&ZERO_INPUT);

    assert!(zero.len() == ACCOUNT_LENGTH);
    assert_digit_zero(zero[5]);

    for index in 0..ACCOUNT_LENGTH {
        assert!(zero[index] == ZERO);
    }
}

fn assert_digit_zero(digit: Digit) {
    assert!(digit.at(0,0) == Cell::Blank);
    assert!(digit.at(0,1) == Cell::Underscore);
    assert!(digit.at(0,2) == Cell::Blank);

    assert!(digit.at(1,0) == Cell::Pipe);
    assert!(digit.at(1,1) == Cell::Blank);
    assert!(digit.at(1,2) == Cell::Pipe);

    assert!(digit.at(2,0) == Cell::Pipe);
    assert!(digit.at(2,1) == Cell::Underscore);
    assert!(digit.at(2,2) == Cell::Pipe);
}

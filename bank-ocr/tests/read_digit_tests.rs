extern crate bank_ocr;

mod input;

use input::*;
use bank_ocr::account::*;
use bank_ocr::reader::*;

#[test]
fn empty_digit_should_contains_blank_only() {
    let empty = Digit::empty();

    for row in 0..3 {
        for col in 0..3 {
            assert!(empty.at(row, col) == Cell::Blank);
        }
    }
}

fn convert_input(input: [&'static str; 4]) -> FileEntry {
    return FileEntry::with_data([input[0].to_string(), input[1].to_string(), input[2].to_string()]);
}

fn assert_account_is_all_digit(account: &Account, digit: Digit) {
    assert!(account.len() == ACCOUNT_LENGTH);
    for index in 0..ACCOUNT_LENGTH {
        assert!(account[index] == digit);
    }
}

#[test]
fn read_account_with_zeros_only_should_succeed() {
    let account_digits = parse(&convert_input(ZERO_INPUT));
    assert_account_is_all_digit(&account_digits, ZERO);

    let account_number = account_digits.value();
    assert!(account_number == Some(0));
}

#[test]
fn read_account_with_ones_only_should_succeed() {
    let account_digits = parse(&convert_input(ONE_INPUT));
    assert_account_is_all_digit(&account_digits, ONE);

    let account_number = account_digits.value();
    assert!(account_number == Some(111111111));
}

#[test]
fn read_account_with_twos_only_should_succeed() {
    let account = parse(&convert_input(TWO_INPUT));
    assert_account_is_all_digit(&account, TWO);
    let account_number = account.value();
    assert!(account_number == Some(222222222));
}

#[test]
fn read_account_with_threes_only_should_succeed() {
    let account = parse(&convert_input(THREE_INPUT));
    assert_account_is_all_digit(&account, THREE);
    let account_number = account.value();
    assert!(account_number == Some(333333333));
}

#[test]
fn read_account_with_fours_only_should_succeed() {
    let account = parse(&convert_input(FOUR_INPUT));
    assert_account_is_all_digit(&account, FOUR);
    let account_number = account.value();
    assert!(account_number == Some(444444444));
}

#[test]
fn read_account_with_fives_only_should_succeed() {
    let account = parse(&convert_input(FIVE_INPUT));
    assert_account_is_all_digit(&account, FIVE);
    let account_number = account.value();
    assert!(account_number == Some(555555555));
}
#[test]
fn read_account_with_sixs_only_should_succeed() {
    let account = parse(&convert_input(SIX_INPUT));
    assert_account_is_all_digit(&account, SIX);
    let account_number = account.value();
    assert!(account_number == Some(666666666));
}
#[test]
fn read_account_with_sevens_only_should_succeed() {
    let account = parse(&convert_input(SEVEN_INPUT));
    assert_account_is_all_digit(&account, SEVEN);
    let account_number = account.value();
    assert!(account_number == Some(777777777));
}
#[test]
fn read_account_with_eights_only_should_succeed() {
    let account = parse(&convert_input(EIGHT_INPUT));
    assert_account_is_all_digit(&account, EIGHT);
    let account_number = account.value();
    assert!(account_number == Some(888888888));
}
#[test]
fn read_account_with_nines_only_should_succeed() {
    let account = parse(&convert_input(NINE_INPUT));
    assert_account_is_all_digit(&account, NINE);
    let account_number = account.value();
    assert!(account_number == Some(999999999));
}

#[test]
fn read_account_123456789_only_should_succeed() {
    let account = parse(&convert_input(INPUT_123456789));

    let account_number = account.value();
    assert!(account_number == Some(123456789));
}
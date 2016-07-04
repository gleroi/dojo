extern crate bank_ocr;

mod input;
mod validation_input;

use validation_input::*;
use bank_ocr::account;
use bank_ocr::checksum::*;

#[test]
fn account_51_should_be_valid() {
    let entry = input::convert_input(FIFTY_ONE);
    let account = account::parse(&entry);
    let state = validate(&account);
    assert_eq!(ValidityState::Valid(EXP_FIFTY_ONE.to_string()), state);
}

#[test]
fn account_49006771X_should_be_illegal() {
    let entry = input::convert_input(ILLEGAL_FOUR);
    let account = account::parse(&entry);
    let state = validate(&account);
    assert_eq!(ValidityState::Illegal(EXP_ILLEGAL_FOUR.to_string()), state);
}

#[test]
fn account_1234X678X_should_be_illegal() {
    let entry = input::convert_input(ILLEGAL_ONE);
    let account = account::parse(&entry);
    let state = validate(&account);
    assert_eq!(ValidityState::Illegal(EXP_ILLEGAL_ONE.to_string()), state);
}
extern crate bank_ocr;

mod input;

use input::{convert_input,ONE_INPUT,THREE_INPUT,SEVEN_INPUT,EIGHT_INPUT};
use bank_ocr::interpolate::*;
use bank_ocr::account::parse;

#[test]
fn all_one_should_interpolate_to_711111111() {
    let account = parse(&convert_input(ONE_INPUT));
    let alternatives = interpolate_account(account);

    assert_eq!(1, alternatives.len());
    assert_eq!(Some(711111111), alternatives[0].value());
}

#[test]
fn all_three_should_interpolate_to_333393333() {
    let account = parse(&convert_input(THREE_INPUT));
    let alternatives = interpolate_account(account);

    assert_eq!(1, alternatives.len());
    assert_eq!(Some(333393333), alternatives[0].value());
}

#[test]
fn all_seven_should_interpolate_to_777777177() {
    let account = parse(&convert_input(SEVEN_INPUT));
    let alternatives = interpolate_account(account);

    assert_eq!(1, alternatives.len());
    assert_eq!(Some(777777177), alternatives[0].value());
}

const TWO_MILLIONS : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ",
" _|| || || || || || || || |",
"|_ |_||_||_||_||_||_||_||_|",
"                           ",
];

#[test]
fn two_hundred_million_should_interpolate_to_200800000() {
    let account = parse(&convert_input(TWO_MILLIONS));
    let alternatives = interpolate_account(account);

    assert_eq!(1, alternatives.len());
    assert_eq!(Some(200800000), alternatives[0].value());
}

const ILLEGAL_ACCOUNT_ONE : [&'static str; 4] = [
"    _  _     _  _  _  _  _ ",
" _| _| _||_||_ |_   ||_||_|",
"  ||_  _|  | _||_|  ||_| _|",
"                           ",
];

#[test]
fn illegal_account_1_should_interpolate_to_123456789() {
    let account = parse(&convert_input(ILLEGAL_ACCOUNT_ONE));
    let alternatives = interpolate_account(account);

    assert_eq!(1, alternatives.len());
    assert_eq!(Some(123456789), alternatives[0].value());
}

const ILLEGAL_ACCOUNT_51 : [&'static str; 4] = [
" _     _  _  _  _  _  _    ",
"| || || || || || || ||_   |",
"|_||_||_||_||_||_||_| _|  |",
"                           ",
];

#[test]
fn illegal_account_51_should_interpolate_to_51() {
    let account = parse(&convert_input(ILLEGAL_ACCOUNT_51));
    let alternatives = interpolate_account(account);

    assert_eq!(1, alternatives.len());
    assert_eq!(Some(51), alternatives[0].value());
}


#[test]
fn all_eight_should_interpolate_to_3_alternatives() {
let account = parse(&convert_input(EIGHT_INPUT));
    let alternatives = interpolate_account(account);

    assert_eq!(3, alternatives.len());
    assert_eq!(Some(888886888), alternatives[0].value());
    assert_eq!(Some(888888988), alternatives[1].value());
    assert_eq!(Some(888888880), alternatives[2].value());
}
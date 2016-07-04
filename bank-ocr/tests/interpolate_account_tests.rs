extern crate bank_ocr;

mod input;

use input::{convert_input,ONE_INPUT,THREE_INPUT};
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

const TWO_MILLIONS : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ",
" _|| || || || || || || || |",
"|_ |_||_||_||_||_||_||_||_|",
"                           ",
];

#[test]
fn tw_hundrer_million_should_interpolate_to_200800000() {
    let account = parse(&convert_input(TWO_MILLIONS));
    let alternatives = interpolate_account(account);

    assert_eq!(1, alternatives.len());
    assert_eq!(Some(200800000), alternatives[0].value());
}
extern crate bank_ocr;

use bank_ocr::account::*;
use bank_ocr::interpolate::*;

#[test]
pub fn zero_alternatives_should_succeed() {
    let alternatives = interpolate_digit(ZERO);

    assert_eq!(alternatives.len(), 1);
    assert_eq!(alternatives[0].value(), Some(8));
}

#[test]
pub fn one_alternatives_should_succeed() {
    let alternatives = interpolate_digit(ONE);

    assert_eq!(alternatives.len(), 1);
    assert_eq!(alternatives[0].value(), Some(7));
}

#[test]
pub fn two_alternatives_should_succeed() {
    let alternatives = interpolate_digit(TWO);

    assert_eq!(alternatives.len(), 0);
}

#[test]
pub fn three_alternatives_should_succeed() {
    let alternatives = interpolate_digit(THREE);

    assert_eq!(alternatives.len(), 1);
    assert_eq!(alternatives[0].value(), Some(9));   
}

#[test]
pub fn four_alternatives_should_succeed() {
    let alternatives = interpolate_digit(FOUR);

    assert_eq!(alternatives.len(), 0);
}

#[test]
pub fn five_alternatives_should_succeed() {
    let alternatives = interpolate_digit(FIVE);

    assert_eq!(alternatives.len(), 2);
    assert_eq!(alternatives[0].value(), Some(9));
    assert_eq!(alternatives[1].value(), Some(6));
}

#[test]
pub fn six_alternatives_should_succeed() {
    let alternatives = interpolate_digit(SIX);

    assert_eq!(alternatives.len(), 2);
    assert_eq!(alternatives[0].value(), Some(8));
    assert_eq!(alternatives[1].value(), Some(5));
}

#[test]
pub fn seven_alternatives_should_succeed() {
    let alternatives = interpolate_digit(SEVEN);

    assert_eq!(alternatives.len(), 1);
    assert_eq!(alternatives[0].value(), Some(1));
}

#[test]
pub fn eight_alternatives_should_succeed() {
    let alternatives = interpolate_digit(EIGHT);

    assert_eq!(alternatives.len(), 3);
    assert_eq!(alternatives[0].value(), Some(0));
    assert_eq!(alternatives[1].value(), Some(6));
    assert_eq!(alternatives[2].value(), Some(9));
}

#[test]
pub fn nine_alternatives_should_succeed() {
    let alternatives = interpolate_digit(NINE);

    assert_eq!(alternatives.len(), 3);
    assert_eq!(alternatives[0].value(), Some(3));
    assert_eq!(alternatives[1].value(), Some(5));
    assert_eq!(alternatives[2].value(), Some(8));
}

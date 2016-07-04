extern crate bank_ocr;

use bank_ocr::account::*;
use bank_ocr::interpolate::*;

#[test]
pub fn zero_alternatives_should_succeed() {
    let alternatives = interpolate_digit(ZERO);

    assert!(alternatives.len() == 2);
    assert_eq!(alternatives[0].value(), Some(0));
    assert_eq!(alternatives[1].value(), Some(8));
}

#[test]
pub fn one_alternatives_should_succeed() {
    let alternatives = interpolate_digit(ONE);

    assert!(alternatives.len() == 2);
    assert_eq!(alternatives[0].value(), Some(1));
    assert_eq!(alternatives[1].value(), Some(7));
}

#[test]
pub fn two_alternatives_should_succeed() {
    let alternatives = interpolate_digit(TWO);

    assert!(alternatives.len() == 1);
    assert_eq!(alternatives[0].value(), Some(2));
}

#[test]
pub fn three_alternatives_should_succeed() {
    let alternatives = interpolate_digit(THREE);

    assert!(alternatives.len() == 2);
    assert_eq!(alternatives[0].value(), Some(3));
    assert_eq!(alternatives[1].value(), Some(9));
}


#[test]
pub fn five_alternatives_should_succeed() {
    let alternatives = interpolate_digit(FIVE);

    assert!(alternatives.len() == 3);
    assert_eq!(alternatives[0].value(), Some(5));
    assert_eq!(alternatives[1].value(), Some(9));
    assert_eq!(alternatives[2].value(), Some(6));
}

#[test]
pub fn six_alternatives_should_succeed() {
    let alternatives = interpolate_digit(SIX);

    assert!(alternatives.len() == 2);
    assert_eq!(alternatives[0].value(), Some(6));
    assert_eq!(alternatives[1].value(), Some(8));
}

#[test]
pub fn nine_alternatives_should_succeed() {
    let alternatives = interpolate_digit(NINE);

    assert!(alternatives.len() == 2);
    assert_eq!(alternatives[0].value(), Some(9));
    assert_eq!(alternatives[1].value(), Some(8));
}

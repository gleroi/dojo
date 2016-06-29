extern crate bank_ocr;

use bank_ocr::checksum::*;

#[test]
fn checksum_345882865_should_work() {
    assert!(checksum(345882865) == true);
}

#[test]
fn checksum_000000000_should_work() {
    assert!(checksum(000000000) == true);
}

#[test]
fn checksum_111111111_should_fail() {
    assert!(checksum(111111111) == false);
}

#[test]
fn checksum_123456789_should_fail() {
    assert!(checksum(111111111) == false);
}
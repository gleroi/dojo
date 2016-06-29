extern crate bank_ocr;

use bank_ocr::reader::*;

use std::fs::File;

#[test]
fn read_file_one_entry_should_succeed() {
    let input = File::open("tests/files/one_entry.txt").unwrap();
    let entries = read_file(input);

    assert!(entries.len() == 1);

    let entry = &entries[0];
    let account_entry = read_digits(entry);
    let account_number = interpret_digits(account_entry).unwrap();
    assert!(account_number == 123456789);
}
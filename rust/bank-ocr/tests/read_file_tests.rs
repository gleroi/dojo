extern crate bank_ocr;

use bank_ocr::reader::*;
use bank_ocr::account::*;

use std::fs::File;

fn assert_entry(entry: &FileEntry, expected_account: u32) {
    let account_entry = parse(entry);
    let account_number = account_entry.value().unwrap();
    assert!(account_number == expected_account);
}

#[test]
fn read_file_one_entry_should_succeed() {
    let input = File::open("tests/files/one_entry.txt").unwrap();
    let entries = read_file(input);

    assert!(entries.len() == 1);

    assert_entry(&entries[0], 123456789);
}

#[test]
fn read_file_three_entries_should_succeed() {
    let input = File::open("tests/files/three_entries.txt").unwrap();
    let entries = read_file(input);

    assert!(entries.len() == 3);

    assert_entry(&entries[0], 999999999);
    assert_entry(&entries[1], 123456789);
    assert_entry(&entries[2], 51);
}
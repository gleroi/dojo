extern crate bank_ocr;

use bank_ocr::reader::*;

pub fn convert_input(input: [&'static str; 4]) -> FileEntry {
    return FileEntry::with_data([input[0].to_string(), input[1].to_string(), input[2].to_string()]);
}

pub const ZERO_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ",
"| || || || || || || || || |",
"|_||_||_||_||_||_||_||_||_|",
"                           ",
];
pub const ONE_INPUT : [&'static str; 4] = [
"                           ",
"  |  |  |  |  |  |  |  |  |",
"  |  |  |  |  |  |  |  |  |",
"                           ",
];
pub const TWO_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ", 
" _| _| _| _| _| _| _| _| _|",
"|_ |_ |_ |_ |_ |_ |_ |_ |_ ",
"                           ",
];
pub const THREE_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ", 
" _| _| _| _| _| _| _| _| _|",
" _| _| _| _| _| _| _| _| _|",
"                           ",
];
pub const FOUR_INPUT : [&'static str; 4] = [
"                           ",
"|_||_||_||_||_||_||_||_||_|",
"  |  |  |  |  |  |  |  |  |",
"                           ",
];
pub const FIVE_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ",
"|_ |_ |_ |_ |_ |_ |_ |_ |_ ",
" _| _| _| _| _| _| _| _| _|",
"                           ",
];
pub const SIX_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ",
"|_ |_ |_ |_ |_ |_ |_ |_ |_ ",
"|_||_||_||_||_||_||_||_||_|",
"                           ",
];
pub const SEVEN_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ",
"  |  |  |  |  |  |  |  |  |",
"  |  |  |  |  |  |  |  |  |",
"                           ",
];
pub const EIGHT_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ",
"|_||_||_||_||_||_||_||_||_|",
"|_||_||_||_||_||_||_||_||_|",
"                           ",
];
pub const NINE_INPUT : [&'static str; 4] = [
" _  _  _  _  _  _  _  _  _ ", 
"|_||_||_||_||_||_||_||_||_|",
" _| _| _| _| _| _| _| _| _|",
"                           ",
];
pub const INPUT_123456789 : [&'static str; 4] = [
"    _  _     _  _  _  _  _ ",
"  | _| _||_||_ |_   ||_||_|",
"  ||_  _|  | _||_|  ||_| _|",
"                           ",
];

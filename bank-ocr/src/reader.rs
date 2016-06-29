use digits::*;
use std::io::{self, BufReader};
use std::fs::File;

pub fn interpret_digits(entry: [Digit; ACCOUNT_LENGTH]) -> Option<u32> {
    const BASE : u32 = 10;
    let mut power : u32 = 1;
    let mut account : u32 = 0;
    for index in (0..ACCOUNT_LENGTH).rev() {
        if let Some(value) = entry[index].value() {
            account +=  value * power;
            power *= BASE;
        }
        else {
            return None;
        }
    }
    return Some(account);
}

pub fn read_digits(input: &[&str; 4]) -> [Digit; ACCOUNT_LENGTH] {
    let mut result = [new_empty_digit(); ACCOUNT_LENGTH];

    for row in 0..3 {
        let line = input[row];
        for (col_in_row, character) in line.char_indices() {
            let digit_index = col_in_row / DIGIT_WIDTH;
            let col = col_in_row % DIGIT_WIDTH;
            let cell = match character {
                ' ' => Cell::Blank,
                '_' => Cell::Underscore,
                '|' => Cell::Pipe,
                other => panic!("unknown character {}", other),
            };
            result[digit_index].set_at(row, col, cell);
        }
    }
    return result;
}

// pub fn read_file(input: &io::Read) -> Vec<[String; 3]> {
//     let reader = BufReader::new(input);
//     let mut result : Vec<[&str; 3]> = Vec::new();
//     let mut done = false;
//     while !done {
//         let mut entry : [&str; 3];
//         for index in 0..3 {
//             let line = String::with_capacity(28);
//             let readedCount = reader.read_line(&line);
//             match readedCount {
//                 Result(count) => if count >= 27 {
//                     line.truncate(27);
//                     entry[index] = line.as_str();
//                 }
//                 else {
//                     panic!("not enough character ({0} chars)", count)
//                 }
//                 None => break
//             }
//         }
//     }
// }
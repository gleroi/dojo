use digits::*;
use std::io::{BufRead, BufReader};
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

pub fn read_digits(input: &[String; 3]) -> [Digit; ACCOUNT_LENGTH] {
    let mut result = [new_empty_digit(); ACCOUNT_LENGTH];

    for row in 0..3 {
        let line = &input[row];
        assert!(line.len() == 27);
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

pub fn read_file(input: File) -> Vec<[String; 3]> {
    let mut reader = BufReader::new(input);
    let mut result : Vec<[String; 3]> = Vec::new();
    let mut done = false;
    while !done {
        match read_entry(&mut reader) {
            Some(entry) => result.push(entry),
            None => done = true,
        }
    }
    return result;
}

fn read_entry(reader: &mut BufReader<File>) -> Option<[String;3]> {
    let mut entry = [String::new(), String::new(), String::new()];
    for index in 0..3 {
        let mut line = String::with_capacity(28);
        let readed_count = reader.read_line(&mut line);
        match readed_count {
            Ok(count) => if count >= 27 {
                line.truncate(27);
                entry[index] = line;
            }
            else {
                println!("not enough character ({0} chars)", count);
                return None;
            },
            Err(error) => {
                println!("error in read_entry: {}", error);
                return None;
            }
        }
    }
    return Some(entry);
}
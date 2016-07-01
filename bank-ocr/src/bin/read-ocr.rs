extern crate bank_ocr;

use std::env as env;
use std::fs::File;

use bank_ocr::reader::*;
use bank_ocr::digits::*;

fn main() {
    let arguments : Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        println!("usage: read-ocr <path>");
        return;
    }

    match File::open(&arguments[1]) {
        Ok(file) => {
            process_file(file);
        }
        Err(error) => {
            panic!("error: {0:9}", error);
        }
    }
}

fn process_file(file: File) {
    let entries = read_file(file);

    for entry in &entries {
        let digits = read_digits(entry);
        if let Some(account) = interpret_digits(digits) {
            println!("{0:09}", account);        
        }
    }
}
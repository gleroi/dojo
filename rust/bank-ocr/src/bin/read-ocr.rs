extern crate bank_ocr;

use std::env as env;
use std::fs::File;

use bank_ocr::reader::*;
use bank_ocr::account::*;
use bank_ocr::checksum::*;

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
            panic!("error: {0}", error);
        }
    }
}

fn process_file(file: File) {
    let accounts = AccountFile::new(file);

    for account in accounts {
        let state = validate(&account);
        println!("{}", state.description());
    }
}
extern crate bank_ocr;

use std::env as env;
use std::fs::File;

use bank_ocr::reader::*;

fn main() {
    let arguments = env::args();

    if arguments.count() != 1 {
        println!("usage: read-ocr <path>");
    }
}
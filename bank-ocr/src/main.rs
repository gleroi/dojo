extern crate bank_ocr;

use bank_ocr::checksum::*;

fn main() {
    println!("Coucou ! {} {}", checksum(345882865), checksum(345882111));
}
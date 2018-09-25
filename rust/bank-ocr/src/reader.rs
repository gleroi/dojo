use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};
use std::fs::File;

pub struct FileEntry {
    data: [String; 3],
}

impl FileEntry {
    pub fn new() -> FileEntry {
        return FileEntry {
            data: [String::new(), String::new(), String::new()]
        }
    }

    pub fn with_data(data: [String; 3]) -> FileEntry {
        return FileEntry {
            data: data
        };
    }
}

impl Index<usize> for FileEntry {
    type Output = String;

    fn index(&self, index: usize) -> &String {
        return &self.data[index];
    }
}

impl IndexMut<usize> for FileEntry {
    fn index_mut(&mut self, index: usize) -> &mut String {
        return &mut self.data[index];
    }
}

pub struct AccountFile {
    reader: BufReader<File>,
}

impl AccountFile {
    pub fn new(input: File) -> AccountFile {
        AccountFile { reader: BufReader::new(input), }
    }
}

use account;
use account::Account;

impl Iterator for AccountFile {
    type Item = Account;

    fn next(&mut self) -> Option<Account> {
        if let Some(entry) = read_entry(&mut self.reader) {
            return Some(account::parse(&entry));
        }
        return None;
    }
}

pub fn read_file(input: File) -> Vec<FileEntry> {
    let mut reader = BufReader::new(input);
    let mut result : Vec<FileEntry> = Vec::new();
    let mut done = false;
    while !done {
        match read_entry(&mut reader) {
            Some(entry) => result.push(entry),
            None => done = true,
        }
    }
    return result;
}

fn read_entry(reader: &mut BufReader<File>) -> Option<FileEntry> {
    let mut entry = FileEntry::new();
    for index in 0..3 {
        let mut line = String::with_capacity(28);
        let readed_count = reader.read_line(&mut line);
        match readed_count {
            Ok(count) => if count >= 27 {
                line.truncate(27);
                entry[index] = line;
            }
            else {
                return None;
            },
            Err(error) => {
                panic!("error in read_entry: {}", error);
            }
        }
    }
    skip_blank_line(reader);    
    return Some(entry);
}

fn skip_blank_line(reader: &mut BufReader<File>) {
    let mut line = String::with_capacity(28);
    if let Err(error) = reader.read_line(&mut line) {
        panic!("error in read_entry: {}", error);
    }
}
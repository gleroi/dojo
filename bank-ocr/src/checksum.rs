use account::*;

pub fn checksum(account: u32) -> bool {
    const BASE : u32 = 10;

    let mut rest = account;
    let mut checksum : u32 = 0;
    for multiplcator in 1..10 {
        let digit = rest % BASE;
        rest = rest / BASE;
        checksum += multiplcator * digit;
    }
    return checksum % 11 == 0;
}

#[derive(PartialEq, Debug)]
pub enum ValidityState {
    Valid(String),
    Illegal(String),
    Error(String),
}

impl ValidityState {
    pub fn description(&self) -> String {
        match *self {
            ValidityState::Valid(ref value) => format!("{}", value),
            ValidityState::Illegal(ref value) => format!("{} {}", value, "ILL"),
            ValidityState::Error(ref value) => format!("{} {}", value, "ERR"),
        }
    }
}

pub fn validate(account: &Account) -> ValidityState {
    let value = account.value();
    let description = account.description();
    match value {
        None => ValidityState::Illegal(description),
        Some(value) => if checksum(value) {
            ValidityState::Valid(description)
        }
        else {
            ValidityState::Error(description)
        }
    }
}


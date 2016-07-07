use account::*;
use interpolate::*;

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

/// represents the different validity state an account number can be.
#[derive(PartialEq, Debug)]
pub enum ValidityState {
    /// Valid state
    /// all digits are valid and the account number satisfies the checksum    
    Valid(String),
    /// Illegale state
    /// some digits are invalid, i.e not [0-9]
    Illegal(String),
    /// Error state
    /// all digits are valid but the account does not satisfy the checksum
    Error(String),
    /// Maybe state
    /// the readed account is either illegal or in error, and multiple alternatives 
    /// valid numbers could be determine
    Maybe(String, Vec<String>),
}

impl ValidityState {
    pub fn description(&self) -> String {
        match *self {
            ValidityState::Valid(ref value) => format!("{}", value),
            ValidityState::Illegal(ref value) => format!("{} {}", value, "ILL"),
            ValidityState::Error(ref value) => format!("{} {}", value, "ERR"),
            ValidityState::Maybe(ref value, ref alts) => format!("{} AMB [{}]", value, alts.join(","))
        }
    }
}

pub fn validate(account: &Account) -> ValidityState {
    let value = account.value();
    let description = account.description();
    match value {
        Some(value) if checksum(value) => ValidityState::Valid(description),
        other => {
            let alternatives = interpolate_account(account);
            if alternatives.len() == 0 {
                match other {
                    None => return ValidityState::Illegal(description),
                    Some(_) => return ValidityState::Error(description)
                }
            }
            if alternatives.len() == 1 {
                let description = alternatives[0].description();
                return ValidityState::Valid(description);
            }
            let altDescriptions = alternatives.iter().map(|altAccount: &Account| { altAccount.description() }).collect();
            return ValidityState::Maybe(description, altDescriptions);
        }
    }
}



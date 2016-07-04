use account::*;
use checksum::*;

pub fn interpolate_account(account: Account) -> Vec<Account> {
    let state = validate(&account);
    match state {
        ValidityState::Error(_) => interpolate_valid_digits(account),
        ValidityState::Illegal(_) => panic!("not implemented"),
        _ => vec!{ account.clone() },
    }
}

pub fn interpolate_valid_digits(account: Account) -> Vec<Account> {
    let mut result : Vec<Account> = Vec::new();

    for index in 0..ACCOUNT_LENGTH {
        let index_digit = account[index];
        let index_alternatives = interpolate_digit(index_digit);
        let mut acc : Account = account.clone();
        for digit_alt in index_alternatives {
            acc[index] = digit_alt;
            if let Some(value) = acc.value() {
                if checksum(value) {
                    result.push(acc.clone());
                }
            }
        }
    }
    return result;
}

pub fn interpolate_digit(input : Digit) -> Vec<Digit> {
    let mut alternatives : Vec<Digit> = Vec::new();
    let mut digit = input;

    push_if_valid(&mut alternatives, &digit);

    for row in 0..DIGIT_HEIGHT {
        for col in 0..DIGIT_WIDTH {
            let original = digit.at(row, col);
            if original == Cell::Blank {
                digit.set_at(row, col, Cell::Pipe);
                push_if_valid(&mut alternatives, &digit);

                digit.set_at(row, col, Cell::Underscore);
                push_if_valid(&mut alternatives, &digit);

                digit.set_at(row, col, Cell::Blank);
            }
        }
    }
    return alternatives;
}

fn push_if_valid(alternatives: &mut Vec<Digit>, digit: &Digit) {
    if digit.value() != None {
        alternatives.push(*digit);
    }
}
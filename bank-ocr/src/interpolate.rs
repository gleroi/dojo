use account::*;

///
///
/// # Examples
///
///
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
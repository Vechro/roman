mod test;

use std::cmp;

const fn roman_lut(numeral: &char) -> Option<u16> {
    match numeral {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _ => None,
    }
}

const fn arabic_lut(digit: &u16) -> Option<&str> {
    match digit {
        1 => Some("I"),
        4 => Some("IV"),
        5 => Some("V"),
        9 => Some("IX"),
        10 => Some("X"),
        40 => Some("XL"),
        50 => Some("L"),
        90 => Some("XC"),
        100 => Some("C"),
        400 => Some("CD"),
        500 => Some("D"),
        900 => Some("DM"),
        1000 => Some("M"),
        _ => None,
    }
}

static DIGITS_DESC: [u16; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

struct Tally {
    total: u64,
    max: u64,
}

// Impure function as it prints to stdout immediately.
pub fn convert_and_print_numerals(list_of_numerals: &[String]) {
    for number_str in list_of_numerals {
        let result = match number_str.chars().next() {
            Some(c) => match c {
                c if c.is_ascii_alphabetic() => roman_to_arabic(&number_str.to_ascii_uppercase()),
                c if c.is_ascii_digit() => arabic_to_roman(number_str),
                _ => None,
            },
            _ => unreachable!(),
        };

        match result {
            Some(s) => println!("{}", s),
            None => println!("Invalid numerals!"),
        };
    }
}

fn arabic_to_roman(arabic_numerals: &str) -> Option<String> {
    let mut num = match arabic_numerals.parse::<u64>() {
        Ok(n) => n,
        Err(_) => return None,
    };

    let result = DIGITS_DESC
        .iter()
        .fold(String::new(), |mut state: String, digit| {
            let quot = num / *digit as u64;
            num = num % *digit as u64;

            let numeral = match arabic_lut(digit) {
                Some(s) => s,
                None => unreachable!(),
            };

            state.push_str(&numeral.repeat(quot as usize));
            state
        });

    Some(result)
}

fn roman_to_arabic(roman_numerals: &str) -> Option<String> {
    let result = roman_numerals.chars().rfold(
        Some(Tally { total: 0, max: 0 }),
        |tally: Option<Tally>, c| {
            let current_value = match roman_lut(&c) {
                Some(val) => val as u64,
                None => return None,
            };

            let (total, mut max) = match tally {
                Some(Tally { total, max }) => (total, max),
                None => return None,
            };

            max = cmp::max(current_value, max);

            if current_value >= max {
                Some(Tally {
                    total: total + current_value,
                    max,
                })
            } else {
                Some(Tally {
                    total: total - current_value,
                    max,
                })
            }
        },
    );

    match result {
        Some(Tally { total, .. }) => Some(total.to_string()),
        None => None,
    }
}

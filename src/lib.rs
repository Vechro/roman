mod test;

use std::cmp;

const ERROR_COPY: &str = "Invalid numerals!";

const fn roman_lut(numeral: &char) -> Option<u16> {
    match numeral {
        'i' => Some(1),
        'v' => Some(5),
        'x' => Some(10),
        'l' => Some(50),
        'c' => Some(100),
        'd' => Some(500),
        'm' => Some(1000),
        _ => None,
    }
}

const fn arabic_lut(digit: &u16) -> Option<&str> {
    match digit {
        1 => Some("i"),
        4 => Some("iv"),
        5 => Some("v"),
        9 => Some("ix"),
        10 => Some("x"),
        40 => Some("xl"),
        50 => Some("l"),
        90 => Some("xc"),
        100 => Some("c"),
        400 => Some("cd"),
        500 => Some("d"),
        900 => Some("dm"),
        1000 => Some("m"),
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
        match number_str.chars().next() {
            Some(c) => match c {
                c if c.is_alphabetic() => {
                    match roman_to_arabic(number_str) {
                        Some(n) => println!("{}", n),
                        None => println!("{}", ERROR_COPY),
                    };
                }
                c if c.is_numeric() => {
                    match arabic_to_roman(number_str) {
                        Some(s) => println!("{}", s),
                        None => println!("{}", ERROR_COPY),
                    };
                }
                _ => return,
            },
            _ => return,
        }
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

fn roman_to_arabic(roman_numerals: &str) -> Option<u64> {
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
        Some(Tally { total, .. }) => Some(total),
        None => None,
    }
}

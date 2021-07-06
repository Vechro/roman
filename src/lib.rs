mod test;

use std::{cmp};

use phf::{phf_map, phf_ordered_map};

const ERROR_COPY: &str = "Invalid numerals!";

static ROMAN_MAP: phf::Map<char, u16> = phf_map! {
    'i' => 1,
    'v' => 5,
    'x' => 10,
    'l' => 50,
    'c' => 100,
    'd' => 500,
    'm' => 1000,
};

static ARABIC_MAP: phf::OrderedMap<u16, &'static str> = phf_ordered_map! {
    1000u16 => "m",
    900u16 => "dm",
    500u16 => "d",
    400u16 => "cd",
    100u16 => "c",
    90u16 => "xc",
    50u16 => "l",
    40u16 => "xl",
    10u16 => "x",
    9u16 => "ix",
    5u16 => "v",
    4u16 => "iv",
    1u16 => "i"
};
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
                    match arabic_to_roman(number_str){
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

    let result = ARABIC_MAP
        .entries()
        .fold(String::new(), |mut state: String, (key, numeral)| {
            let quot = num / *key as u64;
            num = num % *key as u64;

            state.push_str(&numeral.repeat(quot as usize));
            state
        });

    Some(result)
}

fn roman_to_arabic(roman_numerals: &str) -> Option<u64> {
    let result = roman_numerals.chars().rfold(
        Some(Tally { total: 0, max: 0 }),
        |tally: Option<Tally>, c| {
            let current_value = match ROMAN_MAP.get(&c) {
                Some(val) => *val as u64,
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

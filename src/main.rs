use std::env;

use roman::convert_and_print_numerals;

const HELP_COPY: &str = "Please specify either one or multiple:
    a) Roman numerals to be converted to Arabic numerals
    b) Arabic numerals to be converted to Roman numerals";

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 | 1 => println!("{}", HELP_COPY),
        _ => convert_and_print_numerals(&args[1..]),
    }
}

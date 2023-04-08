/// HW1
/// Bradley Thompson
use bradleys_random_rust_helpers::*;
use std::str::FromStr;
use text_colorizer::{Color, *};

const LINE_LEN: u8 = 36;

fn main() {
    println!("Hello, welcome to Homework 1!");
    horizontal_sep(LINE_LEN, Some(Color::Green));

    let args: Vec<String> = std::env::args().skip(1).collect(); // Skip the program name

    if args.len() != 3 {
        error()
    }

    println!("Trying modexp({})...", args.join(","));

    // Note: parse_num restrains input to target bounds implicitly b/c it casts to u64.
    let parsed: Vec<u64> = args.iter().map(|str| parse_num(str)).collect();

    let exp_modulo = modexp(parsed[0], parsed[1], parsed[2]);
    println!("Result: {}", exp_modulo);
    horizontal_sep(LINE_LEN, Some(Color::Green));
}

fn modexp(x: u64, y: u64, m: u64) -> u64 {
    0
}

/// Print a usage error message and exit (essentially what's from HW1 handout).
fn error() -> ! {
    let error_msg = "modexp: usage: modexp <x> <y> <m>".bright_red();
    eprintln!("{error_msg}");
    std::process::exit(1);
}

/// Helper to parse a string slice as a u64, or panic!
fn parse_num<T: FromStr>(s: &str) -> T {
    // TODO: Should prob just move this to my helper lib
    match s.parse() {
        Ok(num) => num,
        Err(_) => {
            let parse_error = format!("Invalid input detected: {}", s).bright_red();
            eprintln!("{parse_error}");
            panic!("Invalid input");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num::<u64>("12"), 12);
        assert_eq!(parse_num::<i8>("-2"), -2);
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn test_parse_num_handles_invalid_input() {
        // Note: see https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic
        parse_num::<u64>("hello world");
    }
}

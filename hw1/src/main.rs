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

/// Function that should efficiently calculate: x^y % m
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    assert!(m != 0);

    let (mut x, mut y) = (x, y);
    if m == 1 {
        0
    } else {
        let mut z = 1_u64;
        while y > 0 {
            if (y % 2) == 1 {
                z = (z % m) * (x % m) % m
            }
            y = y / 2;
            x = x.pow(2) % m
        }
        z
    }
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
    fn test_modexp() {
        assert_eq!(modexp(2, 20, 17), 16);
        assert_eq!(modexp(2, 2, 1), 0);
        assert_eq!(modexp(2, 40, 20), 16);
        assert_eq!(modexp(1_640, 100, 3), 1);
        assert_eq!(modexp(1_000_000_000, 500, 99_999_999), 10_000);
    }

    #[test]
    fn test_modexp_from_handout() {
        // Largest prime less than 2**64.
        // https://primes.utm.edu/lists/2small/0bit.html
        let bigm = u64::max_value() - 58;
        assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
        assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
        assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
        // https://practice.geeksforgeeks.org/problems/
        //    modular-exponentiation-for-large-numbers/0
        assert_eq!(4, modexp(10, 9, 6));
        assert_eq!(34, modexp(450, 768, 517));
    }

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

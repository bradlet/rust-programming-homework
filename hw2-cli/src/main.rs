use bradleys_random_rust_helpers::{horizontal_sep, parse_num};
use hw2::{decrypt, encrypt, genkey};
use text_colorizer::Color;

fn main() {
    println!("Simple RSA Encryption");
    horizontal_sep(42, Some(Color::BrightGreen));

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        panic!("Missing arguments...")
    }

    match args[0].as_str() {
        "genkey" => {
            let key = genkey();
            let (p, q) = (u64::from(key.0), u64::from(key.1));
            println!("New key: p={} q={} pubkey={}", p, q, p * q)
        }
        "encrypt" => {
            if args.len() < 3 {
                panic!("Missing arguments...")
            }
            let public_key = parse_num::<u64>(args[1].as_str());
            let msg = parse_num(args[2].as_str());
            let cyphertext = encrypt(public_key, msg);
            println!("Cyphertext = {}", cyphertext);
        }
        "decrypt" => {
            if args.len() < 4 {
                panic!("Missing arguments...")
            }
            let p = parse_num(args[1].as_str()); // p
            let q = parse_num(args[2].as_str()); // q
            let cyphertext = parse_num(args[3].as_str());
            let plaintext = decrypt((p, q), cyphertext);
            println!("Decrypted msg = {}", plaintext);
        }
        _ => panic!("Invalid selection..."),
    }
    horizontal_sep(42, Some(Color::BrightGreen));
}

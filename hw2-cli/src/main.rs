use bradleys_random_rust_helpers::{horizontal_sep, parse_num};
use hw2::{encrypt, genkey, decrypt};
use text_colorizer::Color;

fn main() {
    println!("Simple RSA Encryption");
    horizontal_sep(36, Some(Color::BrightGreen));

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        panic!("Missing arguments...")
    }

    match args[0].as_str() {
        "genkey" => {
            let key = genkey();
            println!("New key: p={} q={}", key.0, key.1)
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
            let public_key = parse_num(args[1].as_str()); // p
            let private_key = parse_num(args[2].as_str()); // q
            let cyphertext = parse_num(args[3].as_str());
            let plaintext = decrypt((public_key, private_key), cyphertext);
            println!("Decrypted msg = {}", plaintext);
        }
        _ => panic!("Invalid selection..."),
    }
    horizontal_sep(36, Some(Color::BrightGreen));
}

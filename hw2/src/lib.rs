//! lib.rs
//! HW2
//! Author: Bradley Thompson
//! CS 510 Rust Programming (Spring 2023) - Bart Massey
use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Returns the least common multiple between two numbers
fn charmichaels_totient(p: u64, q: u64) -> u64 {
    lcm(p - 1, q - 1)
}

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
///
/// # Output
/// A tuple consisting of two u32 which makeup a private key, and whose product act as the pubkey.
pub fn genkey() -> (u32, u32) {
    loop {
        let (p, q) = (rsa_prime(), rsa_prime());
        let ct = charmichaels_totient(u64::from(p), u64::from(q));
        if (EXP < ct) && (gcd(EXP, ct) == 1) {
            return (p, q);
        }
    }
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
///
/// # Arguments
/// - `key`: The public key; product of the two private key components output by `genkey()`.
/// - `msg`: The plaintext to be encrypted into cyphertext
///
/// # Output
/// A u64 which is the encrypted msg known as the 'cyphertext'.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(u64::from(msg), EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
///
/// # Arguments
/// - `key`: A tuple consisting of both components of the generated private key.
/// - `msg`: The cyphertext to be decrypted into a plaintext message.
///
/// # Output
/// The original u32 plaintext message that was fed into `encrypt`.
///
/// # Panics
/// Panic if decrypted message can't fit into u32.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let (p, q) = (u64::from(key.0), u64::from(key.1));
    let ct = charmichaels_totient(p, q);
    let d = modinverse(EXP, ct);
    u32::try_from(modexp(msg, d, p * q)).unwrap()
}

#[cfg(test)]
mod libtests {
    use super::*;

    #[test]
    fn test_charmichaels_totient() {
        let inputs = [(2, 3), (4, 10), (11, 13)];
        let expectations = [2, 9, 60];
        for i in 0..2 {
            let (p, q) = inputs[i];
            assert_eq!(charmichaels_totient(p, q), expectations[i])
        }
    }

    #[test]
    fn test_encrypt_provided() {
        let result = encrypt(0xde9c5816141c8ba9, 0x12345f);
        assert_eq!(result, 0x6418280e0c4d7675)
    }

    #[test]
    fn test_decrypt_provided() {
        let result = decrypt((0xed23e6cd, 0xf050a04d), 0x6418280e0c4d7675);
        assert_eq!(result, 0x12345f)
    }
}

use toy_rsa_lib::*;
/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Returns the least common multiple between two numbers
fn charmichaels_totient(p: u64, q: u64) -> u64 {
    lcm(p-1, q-1)
}

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
// pub fn genkey() -> (u32, u32) {}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(u64::from(msg), EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
// pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lambda() {
        let inputs = [(2, 3), (4, 10), (11, 13)];
        let expectations = [2, 9, 60];
        for i in 0..2 {
            let (p, q) = inputs[i];
            assert_eq!(charmichaels_totient(p, q), expectations[i])
        }
    }

    #[test]
    fn test_encrypt_provided() {
        // Private Key: p = 0xed23e6cd q = 0xf050a04d
        // Decrypted: 0x12345f

        let result = encrypt(0xde9c5816141c8ba9, 0x12345f);
        assert_eq!(result, 0x6418280e0c4d7675)
    }
}

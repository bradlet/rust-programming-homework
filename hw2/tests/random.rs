use hw2::*;
use rand;

const NUM_TEST_KEYS: usize = 100;
const NUM_TEST_MSGS: usize = 100;

#[test]
fn test_message_information_not_lost() {
    let keys: [(u32, u32); NUM_TEST_KEYS] = [(0, 0); NUM_TEST_KEYS];
    let keys = keys.map(|_| genkey());
    for (p, q) in keys {
        let msgs: [u32; NUM_TEST_MSGS] = [0; NUM_TEST_MSGS];
        let msgs = msgs.map(|_| rand::random());
        let pubkey = u64::from(p) * u64::from(q);
        for msg in msgs {
            let encrypted = encrypt(pubkey, msg);
            let decrypted = decrypt((p, q), encrypted);
            assert_eq!(msg, decrypted);
        }
    }
}

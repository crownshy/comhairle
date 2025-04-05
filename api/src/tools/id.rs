/// Generates random ids for pseudonymous users.

use rand::Rng;

// Cockford base32 encoding
// see https://en.wikipedia.org/wiki/Base32#Crockford's_Base32
const BASE32_ALPHABET: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
    'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
];
const ID_LENGTH: usize = 6;

fn next_char() -> char {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..BASE32_ALPHABET.len());
    BASE32_ALPHABET[index]
}

pub fn gen_id () -> String {
    let mut id = String::new();
    while id.len() < ID_LENGTH {
        let candidate = id.clone() + &next_char().to_string();
        if has_too_many_consecutive_letters(&candidate) {
            continue;
        };
        if has_meaningful_combo(&candidate) {
            continue;
        };
        id = candidate;
    }
    id
}

// These combinations can be meaningful, which increases the risk of something offensive
const MEANINGFUL_COMBOS: [&str; 33] = [
    "UK", "UC", "0C", "0K", "0G", "0I", "0L", "0T", "0X",
    "1C", "1K", "1X", "IX", "IC", "IK",
    "EX", "3X", "XY", 
    "A5", "AS", "AZ",
    "1G", "IG", 
    "H1T", "HT",
    "1S", "IS",
    "5LT",
    "B00", "BO0", "B0O",
    "666", "69",
];

fn has_meaningful_combo(input: &str) -> bool {
    MEANINGFUL_COMBOS.iter().any(|&word| {
        let word = word.to_string();
        input.contains(&word)
    })
}

fn has_too_many_consecutive_letters(input: &str) -> bool {
    let mut count = 0;
    
    for c in input.chars() {
        if c.is_alphabetic() {
            count += 1;
            if count > 2 {
                return true;
            }
        } else {
            count = 0;
        }
    }
    false
}


mod tests {
    use crate::tools::id::{gen_id, has_too_many_consecutive_letters, has_meaningful_combo};

    #[test]
    fn test_gen_id() {
        let id = gen_id();
        assert_eq!(id.len(), 6);
        assert!(id.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_has_too_many_consecutive_letters() {
        assert!(!has_too_many_consecutive_letters("a1c"));
        assert!(has_too_many_consecutive_letters("3abc"));
    }

    #[test]
    fn test_has_meaningful_combos() {
        assert!(has_meaningful_combo("L0CK"));
        assert!(!has_meaningful_combo("HELLO"));
    }
}
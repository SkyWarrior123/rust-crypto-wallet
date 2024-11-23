use primitive_types::U512;

const BASE58_CHARS: [char; 58] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

pub fn convert_hex_to_base58(s: &str) -> String {
    let b = U512::from_big_endian(&[58]);
    let z = U512::from_big_endian(&[0]);

    let mut n = U512::from_str_radix(s, 16).expect("parse-hex");

    let mut r: Vec<u8> = vec![];

    while n > z {
        let rem = n.checked_rem(b).expect("remainder").byte(0);
        r.push(rem);
        n = n.checked_div(b).expect("division");
    }

    let mut rs = r.into_iter().map(|b| BASE58_CHARS[b as usize]).collect::<String>();
    rs = rs.chars().rev().collect::<String>();
    rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_hex_conversion() {
        // Hexadecimal for 1234
        let hex = "4d2";
        let base58 = convert_hex_to_base58(hex);
        assert_eq!(base58, "NH");
    }

    #[test]
    fn test_empty_input() {
        // Empty input should panic or throw an error
        let result = std::panic::catch_unwind(|| convert_hex_to_base58(""));
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_conversion() {
        // Hexadecimal for 0
        let hex = "0";
        let base58 = convert_hex_to_base58(hex);
        assert_eq!(base58, "");
    }

    #[test]
    fn test_single_digit() {
        // Hexadecimal for 1
        let hex = "1";
        let base58 = convert_hex_to_base58(hex);
        assert_eq!(base58, "2");
    }

    #[test]
    fn test_large_number_conversion() {
        // A very large hexadecimal number
        let hex = "ffffffffffffffffffffffffffffffff";
        let base58 = convert_hex_to_base58(hex);
        assert_eq!(base58, "2VzHRkNVhb4joCQfBvQgx1TY73X"); // Pre-computed expected result
    }

    #[test]
    fn test_case_insensitive() {
        // Hex input should be case-insensitive
        let hex_lower = "abcdef";
        let hex_upper = "ABCDEF";
        let base58_lower = convert_hex_to_base58(hex_lower);
        let base58_upper = convert_hex_to_base58(hex_upper);
        assert_eq!(base58_lower, base58_upper);
    }

    #[test]
    fn test_invalid_hex_input() {
        // Input with invalid hex characters
        let result = std::panic::catch_unwind(|| convert_hex_to_base58("g123")); // 'g' is not a valid hex digit
        assert!(result.is_err());
    }

    #[test]
    fn test_leading_zeros() {
        // Hexadecimal input with leading zeros
        let hex = "00004d2";
        let base58 = convert_hex_to_base58(hex);
        assert_eq!(base58, "NH"); // Leading zeros don't affect the number value
    }

    #[test]
    fn test_minimal_length_hex() {
        // Minimal valid hex input
        let hex = "1";
        let base58 = convert_hex_to_base58(hex);
        assert_eq!(base58, "2");
    }

    #[test]
    fn test_nonzero_leading_zero_effect() {
        // Hex input "00a" should behave the same as "a"
        let hex_with_leading_zero = "00a";
        let hex_without_leading_zero = "a";
        let base58_with_leading_zero = convert_hex_to_base58(hex_with_leading_zero);
        let base58_without_leading_zero = convert_hex_to_base58(hex_without_leading_zero);
        assert_eq!(base58_with_leading_zero, base58_without_leading_zero);
    }
}

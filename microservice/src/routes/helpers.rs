use core::panic;
use std::cmp::Ordering;

use chrono::DateTime;
use multiversx_sc_snippets::imports::RustBigUint;

pub fn denominate(value: f64) -> String {
    print!("VALUE IS: {value}");
    if value < 0.0 {
        panic!("Negative values are not allowed.");
    }
    if value == 0.0 {
        return "0".to_string();
    }

    let mut nominated_value = value.to_string();
    match nominated_value.chars().next() {
        Some('0') => {
            if nominated_value.chars().nth(1).unwrap() != '.' {
                return "0".to_string();
            }
        }
        Some('-') => {
            panic!("Negative values are not allowed.");
        }
        _ => {}
    }

    if nominated_value.contains('.') {
        let split_nominated: Vec<&str> = nominated_value.split('.').collect();
        if split_nominated.len() != 2 {
            panic!("Invalid nominated value.");
        } else {
            let integer_part = split_nominated[0].to_string();
            let mut decimal_part = split_nominated[1].to_string();

            match 18usize.cmp(&decimal_part.len()) {
                Ordering::Less => {
                    decimal_part = decimal_part[..18].to_string();
                }
                Ordering::Greater => {
                    let zeros_left = 18usize - decimal_part.len();
                    decimal_part.push_str(&"0".repeat(zeros_left));
                }
                Ordering::Equal => {}
            }

            let result = integer_part + &decimal_part;
            result.trim_start_matches('0').to_string()
        }
    } else {
        nominated_value.push_str(&"0".repeat(18));
        nominated_value
    }
}

pub fn nominated_str(value: RustBigUint) -> String {
    let string_value = value.to_string();

    if string_value.len() <= 18usize {
        if string_value.chars().all(|c| c == '0') {
            return "0".to_string();
        }
        let mut result = "0.".to_string();
        let len_diff = 18usize - string_value.len();
        println!("{}", len_diff);
        result.push_str(&"0".repeat(len_diff));
        result.push_str(string_value.trim_end_matches("0"));
        result
    } else {
        let len_diff = string_value.len() - 18usize;
        let mut result = string_value[..len_diff].to_string();

        if string_value[len_diff..len_diff + 4]
            .chars()
            .all(|c| c == '0')
        {
            result.push_str(".0000");
            result
        } else {
            result.push('.');
            result.push_str(&string_value[len_diff..]);
            result.trim_end_matches('0').to_string()
        }
    }
}

pub fn readable_timestamp(timestamp: u64) -> String {
    let datetime =
        DateTime::from_timestamp(timestamp as i64, 0).expect("Failed to parse timestamp");
    datetime.to_string()
}

#[test]
fn test_denominate_zero() {
    let value = 0.0;
    let result = denominate(value);
    assert_eq!(result, "0");
}

#[test]
fn test_denominate_positive_integer() {
    let value = 1234.0;
    let result = denominate(value);
    assert_eq!(result, "1234000000000000000000");
}

#[test]
fn test_denominate_positive_float() {
    let value = 12.345678901234567;
    let result = denominate(value);
    assert_eq!(result, "12345678901234567000");
}

#[test]
fn test_denominate_max_precision() {
    let value = 0.12345678901234568;
    let result = denominate(value);
    assert_eq!(result, "123456789012345680");
}

#[tokio::test]
#[should_panic(expected = "Negative values are not allowed.")]
async fn test_denominate_negative_value() {
    let value = -1.0;
    denominate(value);
}

#[test]
fn test_nominated_str_zero() {
    let value = RustBigUint::from(0u32);
    let result = nominated_str(value);
    assert_eq!(result, "0");
}

#[test]
fn test_nominated_str_less_than_18_digits() {
    let value = RustBigUint::from(12345u32);
    let result = nominated_str(value);
    assert_eq!(result, "0.000000000000012345");
}

#[test]
fn test_nominated_str_exactly_18_digits() {
    let value = RustBigUint::parse_bytes(b"123456789012345678", 10).unwrap();
    let result = nominated_str(value);
    assert_eq!(result, "0.123456789012345678");
}

#[test]
fn test_nominated_str_more_than_18_digits() {
    let value = RustBigUint::parse_bytes(b"123456789012345678901234567890", 10).unwrap();
    let result = nominated_str(value);
    assert_eq!(result, "123456789012.34567890123456789");
}

#[test]
fn test_nominated_str_trailing_zeros() {
    let value = RustBigUint::parse_bytes(b"1000000000000000000000", 10).unwrap();
    let result = nominated_str(value);
    assert_eq!(result, "1000.0000");
}

#[test]
fn test_nominated_str_no_decimal() {
    let value = RustBigUint::parse_bytes(b"1000000000000000000000000", 10).unwrap();
    let result = nominated_str(value);
    assert_eq!(result, "1000000.0000");
}

#[test]
fn test_nominate() {
    let mut denominated_value = RustBigUint::from(1_000_000_000u128);
    let result = nominated_str(denominated_value);
    assert_eq!(result, "0.000000001");

    denominated_value = RustBigUint::from(1_000_000_000_000_000_000u128);
    let result = nominated_str(denominated_value);
    assert_eq!(result, "1.0000");

    denominated_value = RustBigUint::from(1_000_000_000_000_000_001u128);
    let result = nominated_str(denominated_value);
    assert_eq!(result, "1.0000");

    denominated_value = RustBigUint::from(1000000000004141411u128);
    let result = nominated_str(denominated_value);
    assert_eq!(result, "1.0000");

    denominated_value = RustBigUint::from(100456231123000000000u128);
    let result = nominated_str(denominated_value);
    assert_eq!(result, "100.456231123");
}

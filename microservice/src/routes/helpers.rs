use chrono::DateTime;
use multiversx_sc_snippets::imports::RustBigUint;

pub fn denominate(value: f64) -> u128 {
    let multiplier: f64 = 10f64.powi(18);
    let result = value * multiplier;

    if result < 0.0 {
        panic!("Negative values are not allowed.");
    }
    if result > u128::MAX as f64 {
        panic!("Result is too large to fit in u128.");
    }

    result as u128
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

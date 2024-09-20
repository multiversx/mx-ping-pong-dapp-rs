use std::cmp::Ordering;

use multiversx_my_sc_snippets::imports::RustBigUint;

pub fn denominate(value: f64) -> String {
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


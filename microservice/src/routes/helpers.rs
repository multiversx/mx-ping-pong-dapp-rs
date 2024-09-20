use actix_web::HttpResponse;
use chrono::DateTime;
use core::panic;
use multiversx_my_sc_snippets::imports::RustBigUint;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub fn readable_timestamp(timestamp: u64) -> String {
    let datetime =
        DateTime::from_timestamp(timestamp as i64, 0).expect("Failed to parse timestamp");
    datetime.to_string()
}

pub fn nominated_str(value: RustBigUint) -> String {
    let string_value = value.to_string();

    if string_value.len() <= 18usize {
        if string_value.chars().all(|c| c == '0') {
            return "0".to_string();
        }
        let mut result = "0.".to_string();
        let len_diff = 18usize - string_value.len();
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

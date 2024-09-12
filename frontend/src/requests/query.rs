use crate::config::Config;

use super::request;

pub enum QueryType {
    Deadline,
    Timestamp,
    MaxFunds,
    PingAmount,
    UserAddresses,
}

pub async fn get_deadline(config: &Config) -> Result<String, String> {
    let query_url = &config.query_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{query_url}");

    request::get_request("deadline", &endpoint).await
}

pub async fn get_timestamp(config: &Config) -> Result<String, String> {
    let query_url = &config.query_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{query_url}");

    request::get_request("timestamp", &endpoint).await
}

pub async fn get_max_funds(config: &Config) -> Result<String, String> {
    let query_url = &config.query_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{query_url}");

    request::get_request("max_funds", &endpoint).await
}

pub async fn get_ping_amount(config: &Config) -> Result<String, String> {
    let query_url = &config.query_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{query_url}");

    request::get_request("ping_amount", &endpoint).await
}

pub async fn get_user_addresses(config: &Config) -> Result<String, String> {
    let query_url = &config.query_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{query_url}");

    request::get_request("user_addresses", &endpoint).await
}
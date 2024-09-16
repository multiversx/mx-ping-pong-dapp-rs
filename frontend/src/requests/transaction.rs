use serde_json::{json, Value};

use crate::config::Config;

use super::request;

pub enum TransactionType {
    Ping,
    Pong,
    PongAll,
}

pub async fn ping(config: &Config) -> Result<Value, Value> {
    let transaction_url = &config.transaction_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{transaction_url}/ping");

    let body = json!({
        "sender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
        "value": 0.001
    });

    request::post_request("ping", &endpoint, Some(&body)).await
}

pub async fn sc_setup(
    config: &Config,
    ping_amount: String,
    max_funds: String,
    activation_timestamp: String,
    duration: String,
) -> Result<Value, Value> {
    let setup_url = &config.setup_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{setup_url}");

    let ping_amount = ping_amount.parse::<f64>().unwrap();
    let max_funds = max_funds.parse::<f64>().unwrap();
    let duration = duration.parse::<u64>().unwrap();

    let body = json!({
        "ping_amount": ping_amount,
        "max_funds": max_funds,
        "activation_timestamp": activation_timestamp,
        "duration": duration,
        "deployer": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
    });

    request::post_request("sc_setup", &endpoint, Some(&body)).await
}

pub async fn pong(config: &Config) -> Result<Value, Value> {
    let transaction_url = &config.transaction_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{transaction_url}/pong");

    request::post_request("pong", &endpoint, None).await
}

pub async fn pong_all(config: &Config) -> Result<Value, Value> {
    let transaction_url = &config.transaction_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{transaction_url}/pong_all");

    request::post_request("pong_all", &endpoint, None).await
}

use crate::config::Config;

use super::request;

pub async fn ping(config: &Config) -> Result<String, String> {
    let transaction_url = &config.transaction_url;
    let dest = &config.dest;
    let full_endpoint = format!("http://{dest}{transaction_url}");

    request::post_request("ping", &full_endpoint).await
}

pub async fn sc_setup(config: &Config) -> Result<String, String> {
    let setup_url = &config.setup_url;
    let dest = &config.dest;
    let full_endpoint = format!("http://{dest}{setup_url}");

    request::post_request("sc_setup", &full_endpoint).await
}

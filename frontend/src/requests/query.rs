use crate::config::Config;

use super::request;

pub async fn get_deadline(config: &Config) -> Result<String, String> {
    let query_url = &config.query_url;
    let dest = &config.dest;
    let full_endpoint = format!("http://{dest}{query_url}/deadline");

    request::get_request("get_deadline", &full_endpoint).await
}

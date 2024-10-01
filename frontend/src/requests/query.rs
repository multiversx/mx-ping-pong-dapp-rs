use crate::{config::Config, requests::ApiResponse};

use super::{request, ContractState};

pub async fn _get_deadline(config: &Config) -> Result<String, String> {
    let query_url = &config.query_url;
    let dest = &config.dest;
    let full_endpoint = format!("http://{dest}{query_url}/deadline");

    let response = request::get_request("get_deadline", &full_endpoint).await;

    match response {
        Ok(value) => match serde_json::from_value::<ApiResponse<String>>(value) {
            Ok(deserialized_value) => Ok(deserialized_value.response),
            Err(e) => Err(e.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}

pub async fn get_contract_state(config: &Config) -> Result<ContractState, String> {
    let query_url = &config.query_url;
    let dest = &config.dest;
    let full_endpoint = format!("http://{dest}{query_url}/contract_state");

    let response = request::get_request("get_contract_state", &full_endpoint).await;

    match response {
        Ok(value) => match serde_json::from_value::<ApiResponse<ContractState>>(value) {
            Ok(deserialized_value) => {
                log::info!("{deserialized_value:?}");
                Ok(deserialized_value.response)
            }
            Err(e) => Err(e.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}

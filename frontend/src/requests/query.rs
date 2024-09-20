use crate::{config::Config, requests::ApiResponse};

use super::{request, ContractState};

pub enum QueryType {
    Deadline,
    Timestamp,
    MaxFunds,
    PingAmount,
    UserAddresses,
}

pub async fn get_contract_addr(config: &Config) -> Result<String, String> {
    let setup_url = &config.setup_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{setup_url}");

    let response = request::get_request("contract_address", &endpoint).await;

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
    let endpoint = format!("http://{dest}{query_url}");

    let response = request::get_request("contract_state", &endpoint).await;

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

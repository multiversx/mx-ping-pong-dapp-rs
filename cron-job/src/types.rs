use multiversx_sc_snippets::imports::StaticApi;
use serde::{Deserialize, Serialize};

use crate::proxy;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractState {
    pub ping_amount: String,
    pub deadline: u64,
    pub activation_timestamp: u64,
    pub max_funds: Option<String>,
    pub pong_all_last_user: usize,
}

impl From<proxy::ContractState<StaticApi>> for ContractState {
    fn from(value: proxy::ContractState<StaticApi>) -> Self {
        ContractState {
            ping_amount: value.ping_amount.to_display().to_string(),
            deadline: value.deadline,
            activation_timestamp: value.activation_timestamp,
            max_funds: value.max_funds.map(|num| num.to_display().to_string()),
            pong_all_last_user: value.pong_all_last_user,
        }
    }
}

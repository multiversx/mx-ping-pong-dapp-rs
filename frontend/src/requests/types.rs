use serde::{Deserialize, Serialize};

pub trait ApiResponseTypes {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ApiResponse<T: ApiResponseTypes> {
    pub response: T,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ContractState {
    pub ping_amount: String,
    pub deadline: u64,
    pub activation_timestamp: u64,
    pub max_funds: Option<String>,
    pub pong_all_last_user: usize,
}

impl ApiResponseTypes for ContractState {}
impl ApiResponseTypes for String {}

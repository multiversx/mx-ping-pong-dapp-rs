// use crate::transactions::helpers::{denominate, nominated_str};
// use multiversx_sc_snippets::imports::RustBigUint;
use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// pub struct PingReqBody {
//     pub value: f64,
// }

// impl PingReqBody {
//     pub fn get_denominated_amount(&self) -> String {
//         denominate(self.value)
//     }
// }

// #[derive(Deserialize, Serialize)]
// pub struct PingResponse {
//     pub response: String,
//     pub amount: String,
// }

// impl PingResponse {
//     pub fn new(response: String, amount: u64) -> Self {
//         Self {
//             response,
//             amount: nominated_str(amount),
//         }
//     }
// }

#[derive(Deserialize, Serialize)]
pub struct SuccessTxResponse {
    pub response: String,
}

impl SuccessTxResponse {
    pub fn new(response: String) -> Self {
        Self { response }
    }
}

use crate::routes::helpers::{denominate, nominated_str};
use actix_web::HttpResponse;
use multiversx_sc_snippets::imports::RustBigUint;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DeployReqBody {
    pub ping_amount: f64,
    pub max_funds: f64,
    pub activation_timestamp: u64,
    pub duration: u64,
}

impl DeployReqBody {
    pub fn get_tx_sending_values(&self) -> (String, String, u64, u64) {
        (
            denominate(self.ping_amount),
            denominate(self.max_funds),
            self.activation_timestamp,
            self.duration,
        )
    }
}

#[derive(Deserialize, Serialize)]
pub struct DeployResponse {
    response: String,
    address: String,
}

impl DeployResponse {
    pub fn new(tx_response: (String, String)) -> Self {
        Self {
            response: tx_response.0,
            address: tx_response.1,
        }
    }

    pub fn send(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PingReqBody {
    pub value: f64,
}

impl PingReqBody {
    pub fn get_denominated_amount(&self) -> String {
        denominate(self.value)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PingResponse {
    response: String,
    amount: String,
}

impl PingResponse {
    pub fn new(response: String, amount: RustBigUint) -> Self {
        Self {
            response,
            amount: nominated_str(amount),
        }
    }

    pub fn send(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SuccessTxResponse {
    response: String,
}

impl SuccessTxResponse {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn send(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

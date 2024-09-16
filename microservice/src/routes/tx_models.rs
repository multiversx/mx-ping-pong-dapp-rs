use crate::routes::helpers::denominate;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DeployReqBody {
    pub ping_amount: f64,
    pub max_funds: f64,
    pub activation_timestamp: String,
    pub duration: u64,
    pub deployer: String,
}

impl DeployReqBody {
    pub fn get_tx_sending_values(&self) -> (u128, u128, String, u64, String) {
        (
            denominate(self.ping_amount),
            denominate(self.max_funds),
            self.activation_timestamp.clone(),
            self.duration,
            self.deployer.clone(),
        )
    }
}

#[derive(Deserialize, Serialize)]
pub struct DeployResponse {
    response: String,
    address: String,
}

#[allow(unused)]
impl DeployResponse {
    pub fn new(tx_response: (String, String)) -> Self {
        Self {
            response: tx_response.0,
            address: tx_response.1,
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PingReqBody {
    pub sender: String,
    pub value: f64,
}

impl PingReqBody {
    pub fn get_tx_sending_values(&self) -> (u128, String) {
        (denominate(self.value), self.sender.clone())
    }
}

#[derive(Deserialize, Serialize)]
pub struct PingResponse {
    response: String,
}

#[allow(unused)]
impl PingResponse {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PongReqBody {
    pub sender: String,
}

impl PongReqBody {
    pub fn get_tx_sending_values(&self) -> String {
        self.sender.clone()
    }
}

#[derive(Deserialize, Serialize)]
pub struct PongResponse {
    response: String,
}

#[allow(unused)]
impl PongResponse {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

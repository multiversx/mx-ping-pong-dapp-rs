use actix_web::HttpResponse;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

pub fn readable_timestamp(timestamp: u64) -> String {
    let datetime =
        DateTime::from_timestamp(timestamp as i64, 0).expect("Failed to parse timestamp");
    datetime.to_string()
}

#[derive(Deserialize, Serialize)]
pub struct QueryResponse {
    response: String,
}

impl QueryResponse {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Deserialize, Serialize)]
pub struct QueryArrayResponse {
    response: Vec<String>,
}

impl QueryArrayResponse {
    pub fn new(response: Vec<String>) -> Self {
        Self { response }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

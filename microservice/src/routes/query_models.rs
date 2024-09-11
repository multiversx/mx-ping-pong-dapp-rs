use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryResponse {
    response: String
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
    response: Vec<String>
}

impl QueryArrayResponse {
    pub fn new(response: Vec<String>) -> Self {
        Self { response }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
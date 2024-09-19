use actix_web::HttpResponse;
use redis::FromRedisValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractState {
    pub ping_amount: String,
    pub deadline: u64,
    pub activation_timestamp: u64,
    pub max_funds: Option<String>,
    pub pong_all_last_user: usize,
}

impl FromRedisValue for ContractState {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::BulkString(ref bytes) => {
                let json_str = core::str::from_utf8(bytes).map_err(|e| {
                    redis::RedisError::from((
                        redis::ErrorKind::TypeError,
                        "Invalid UTF-8 string",
                        e.to_string(),
                    ))
                })?;

                // deserialize the JSON into `ContractState`
                serde_json::from_str(json_str).map_err(|e| {
                    redis::RedisError::from((
                        redis::ErrorKind::TypeError,
                        "Failed to deserialize into ContractState from redis",
                        e.to_string(),
                    ))
                })
            }
            redis::Value::Nil => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Key not found",
            ))),

            // error for unsupported types
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Unexpected Redis value type for ContractState",
            ))),
        }
    }
}

pub trait QueryResponseTypes {}

pub type QuerryArray = Vec<String>;

#[derive(Deserialize, Serialize)]
pub struct QueryResponse<T: QueryResponseTypes> {
    response: T,
}

impl<T: QueryResponseTypes + Serialize> QueryResponse<T> {
    pub fn new(response: T) -> Self {
        Self { response }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl QueryResponseTypes for ContractState {}
impl QueryResponseTypes for String {}
impl QueryResponseTypes for QuerryArray {}

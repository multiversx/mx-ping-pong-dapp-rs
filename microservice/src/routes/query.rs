use actix_web::{get, web, Responder};
use imports::{Bech32Address, ReturnsResultUnmanaged};
use interactor::ContractInteract;
use redis::{AsyncCommands, Client, RedisError};

use crate::routes::{
    helpers::{nominated_str, readable_timestamp},
    proxy,
    query_models::*,
};
use multiversx_sc_snippets::*;

#[get("/deadline")]
pub async fn get_deadline(redis_client: web::Data<Client>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let deadline_cached_value: Result<String, RedisError> = con.get("deadline").await;

    match deadline_cached_value {
        Ok(deadline) => QueryResponse::new(deadline).response(),
        Err(_) => {
            let current_address = contract_interact.state.current_address().clone();

            let result_value = contract_interact
                .interactor
                .query()
                .to(current_address)
                .typed(proxy::PingPongProxy)
                .deadline()
                .returns(ReturnsResultUnmanaged)
                .prepare_async()
                .run()
                .await;

            let formatted_deadline = readable_timestamp(result_value);

            let _: () = con.set("deadline", &formatted_deadline).await.unwrap();
            QueryResponse::new(formatted_deadline).response()
        }
    }
}

#[get("/timestamp")]
pub async fn get_timestamp(redis_client: web::Data<Client>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let timestamp_cached_value: Result<String, RedisError> = con.get("timestamp").await;

    match timestamp_cached_value {
        Ok(timestamp) => QueryResponse::new(timestamp).response(),
        Err(_) => {
            let current_address = contract_interact.state.current_address().clone();

            let result_value = contract_interact
                .interactor
                .query()
                .to(current_address)
                .typed(proxy::PingPongProxy)
                .activation_timestamp()
                .returns(ReturnsResultUnmanaged)
                .prepare_async()
                .run()
                .await;

            let formatted_timestamp = readable_timestamp(result_value);
            let _: () = con.set("timestamp", &formatted_timestamp).await.unwrap();
            QueryResponse::new(formatted_timestamp).response()
        }
    }
}

#[get("/ping_amount")]
pub async fn get_ping_amount(redis_client: web::Data<Client>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let ping_amount_cached_value: Result<String, RedisError> = con.get("ping_amount").await;

    match ping_amount_cached_value {
        Ok(ping_amount) => QueryResponse::new(ping_amount).response(),
        Err(_) => {
            let current_address = contract_interact.state.current_address().clone();

            let result_value = contract_interact
                .interactor
                .query()
                .to(current_address)
                .typed(proxy::PingPongProxy)
                .ping_amount()
                .returns(ReturnsResultUnmanaged)
                .prepare_async()
                .run()
                .await;

            let nominated_result_value = nominated_str(result_value);
            let _: () = con
                .set("ping_amount", &nominated_result_value)
                .await
                .unwrap();
            QueryResponse::new(nominated_result_value).response()
        }
    }
}

#[get("/max_funds")]
pub async fn get_max_funds(redis_client: web::Data<Client>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let max_funds_cached_value: Result<String, RedisError> = con.get("max_funds").await;

    match max_funds_cached_value {
        Ok(max_funds) => QueryResponse::new(max_funds).response(),
        Err(_) => {
            let current_address = contract_interact.state.current_address().clone();

            let result_value = contract_interact
                .interactor
                .query()
                .to(current_address)
                .typed(proxy::PingPongProxy)
                .max_funds()
                .returns(ReturnsResultUnmanaged)
                .prepare_async()
                .run()
                .await;

            match result_value {
                Some(value) => {
                    let nominated_result_value = nominated_str(value);
                    let _: () = con.set("max_funds", &nominated_result_value).await.unwrap();
                    QueryResponse::new(nominated_result_value).response()
                }
                None => QueryResponse::new("Max funds limit not set".to_string()).response(),
            }
        }
    }
}

#[get("/user_addresses")]
pub async fn get_user_addresses(redis_client: web::Data<Client>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let user_addresses_cached_value: Result<String, RedisError> = con.get("user_addresses").await;

    match user_addresses_cached_value {
        Ok(user_addresses_json) => {
            let user_addresses: Vec<String> = serde_json::from_str(&user_addresses_json).unwrap();
            QueryResponse::<QuerryArray>::new(user_addresses).response()
        }
        Err(_) => {
            let current_address = contract_interact.state.current_address().clone();

            let result_value = contract_interact
                .interactor
                .query()
                .to(current_address)
                .typed(proxy::PingPongProxy)
                .get_user_addresses()
                .returns(ReturnsResultUnmanaged)
                .prepare_async()
                .run()
                .await;

            let serializable_result_addresses = result_value
                .iter()
                .map(|addr| Bech32Address::from(addr).to_string())
                .collect();

            let serialized_response =
                serde_json::to_string(&serializable_result_addresses).unwrap();

            let _: () = con
                .set("user_addresses", serialized_response.to_string())
                .await
                .unwrap();

            QueryResponse::<QuerryArray>::new(serializable_result_addresses).response()
        }
    }
}

#[get("/contract_state")]
pub async fn get_contract_state(redis_client: web::Data<Client>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    // already deserialized
    let contract_state_cached_value: Result<ContractState, RedisError> =
        con.get("contract_state").await;

    log::info!("cached from redis {contract_state_cached_value:?}");
    println!("cached from redis {contract_state_cached_value:?}");

    match contract_state_cached_value {
        Ok(value) => QueryResponse::<ContractState>::new(value).response(),
        Err(_) => {
            let current_address = contract_interact.state.current_address().clone();

            let result_value = contract_interact
                .interactor
                .query()
                .to(current_address)
                .typed(proxy::PingPongProxy)
                .get_contract_state()
                .returns(ReturnsResultUnmanaged)
                .prepare_async()
                .run()
                .await;

            let contract_state = ContractState {
                ping_amount: result_value.ping_amount.to_display().to_string(),
                deadline: result_value.deadline,
                activation_timestamp: result_value.activation_timestamp,
                max_funds: result_value
                    .max_funds
                    .map(|num| num.to_display().to_string()),
                pong_all_last_user: result_value.pong_all_last_user,
            };

            let serialized_response = serde_json::to_string(&contract_state).unwrap();

            let _: () = con
                .set("contract_state", serialized_response)
                .await
                .unwrap();

            QueryResponse::<ContractState>::new(contract_state).response()
        }
    }
}

pub fn query_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(get_deadline)
        .service(get_timestamp)
        .service(get_max_funds)
        .service(get_ping_amount)
        .service(get_user_addresses)
        .service(get_contract_state);
}

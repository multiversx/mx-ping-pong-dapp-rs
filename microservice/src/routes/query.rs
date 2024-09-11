use actix_web::{get, web, Responder};
use imports::ReturnsResultUnmanaged;

use crate::routes::{query_models::*,proxy};
use crate::shared_state::AppState;
use multiversx_sc_snippets::*;

#[get("/deadline")]
pub async fn get_deadline(data: web::Data<AppState>) -> impl Responder {
    // get a mutable lock on the contract_interact (entire struct)
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };
    let current_address = contract_interact.state.current_address().clone();

    // access both interactor and state through the mutable borrow
    let result_value = contract_interact
        .interactor
        .query()
        .to(current_address) // access state via the mutable reference
        .typed(proxy::PingPongProxy)
        .deadline()
        .returns(ReturnsResultUnmanaged)
        .prepare_async()
        .run()
        .await;

    format!("Result: {result_value:?}")
}

#[get("/timestamp")]
pub async fn get_timestamp(data: web::Data<AppState>) -> impl Responder {
    // get a mutable lock on the contract_interact (entire struct)
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };
    let current_address = contract_interact.state.current_address().clone();

    // access both interactor and state through the mutable borrow
    let result_value = contract_interact
        .interactor
        .query()
        .to(current_address) // access state via the mutable reference
        .typed(proxy::PingPongProxy)
        .activation_timestamp()
        .returns(ReturnsResultUnmanaged)
        .prepare_async()
        .run()
        .await;

    format!("Result: {result_value:?}")
}

#[get("/ping_amount")]
pub async fn get_ping_amount(data: web::Data<AppState>) -> impl Responder {
    // get a mutable lock on the contract_interact (entire struct)
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };
    let current_address = contract_interact.state.current_address().clone();

    // access both interactor and state through the mutable borrow
    let result_value = contract_interact
        .interactor
        .query()
        .to(current_address) // access state via the mutable reference
        .typed(proxy::PingPongProxy)
        .ping_amount()
        .returns(ReturnsResultUnmanaged)
        .prepare_async()
        .run()
        .await;

    format!("Result: {result_value:?}")
}

#[get("/max_funds")]
pub async fn get_max_funds(data: web::Data<AppState>) -> impl Responder {
    // get a mutable lock on the contract_interact (entire struct)
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };
    let current_address = contract_interact.state.current_address().clone();

    // access both interactor and state through the mutable borrow
    let result_value = contract_interact
        .interactor
        .query()
        .to(current_address) // access state via the mutable reference
        .typed(proxy::PingPongProxy)
        .max_funds()
        .returns(ReturnsResultUnmanaged)
        .prepare_async()
        .run()
        .await;

    format!("Result: {result_value:?}")
}

#[get("/user_addresses")]
pub async fn get_user_addresses(data: web::Data<AppState>) -> impl Responder {
    // get a mutable lock on the contract_interact (entire struct)
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };
    let current_address = contract_interact.state.current_address().clone();

    // access both interactor and state through the mutable borrow
    let result_value = contract_interact
        .interactor
        .query()
        .to(current_address) // access state via the mutable reference
        .typed(proxy::PingPongProxy)
        .get_user_addresses()
        .returns(ReturnsResultUnmanaged)
        .prepare_async()
        .run()
        .await;

    format!("Result: {result_value:?}")
}

pub fn query_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(get_deadline)
        .service(get_timestamp)
        .service(get_max_funds)
        .service(get_ping_amount)
        .service(get_user_addresses);
}

use actix_web::{get, web, Responder};
use imports::{Bech32Address, ReturnsResultUnmanaged};
use interactor::ContractInteract;

use crate::routes::proxy;
use multiversx_sc_snippets::*;

#[get("/deadline")]
pub async fn get_deadline() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
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

    format!("Result: {result_value:?}")
}

#[get("/timestamp")]
pub async fn get_timestamp() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
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

    format!("Result: {result_value:?}")
}

#[get("/ping_amount")]
pub async fn get_ping_amount() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
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

    format!("Result: {result_value:?}")
}

#[get("/max_funds")]
pub async fn get_max_funds() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
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

    format!("Result: {result_value:?}")
}

#[get("/user_addresses")]
pub async fn get_user_addresses() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
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

        let addresses: Vec<String> = result_value
        .iter()
        .map(|address| Bech32Address::from(address).to_string())
        .collect();

    // Return the addresses wrapped in a JSON response
    web::Json(Query { addresses })
}

pub fn query_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(get_deadline)
        .service(get_timestamp)
        .service(get_max_funds)
        .service(get_ping_amount)
        .service(get_user_addresses);
}

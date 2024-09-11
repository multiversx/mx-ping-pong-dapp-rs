use actix_web::{post, web, Responder};
use imports::{Bech32Address, IgnoreValue, ReturnsRawResult};

use crate::routes::{tx_models::*, proxy};
use crate::shared_state::AppState;
use multiversx_sc_snippets::*;

#[post("/ping")]
pub async fn ping(data: web::Data<AppState>, body: web::Json<PingReqBody>) -> impl Responder {
    // get a mutable lock on the contract_interact (entire struct)
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };

    let (amount, sender) = body.get_tx_sending_values();

    let wallet_address = Bech32Address::from_bech32_string(sender);
    let current_address = contract_interact.state.current_address().clone();
    let ping_amount = amount as u64;
    let _data = IgnoreValue;

    // mby unlock if failure ?
    // access both interactor and state through the mutable borrow
    let response = contract_interact
        .interactor
        .tx()
        .from(wallet_address)
        .to(current_address)
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .ping(_data)
        .egld(ping_amount)
        .returns(ReturnsRawResult)
        .prepare_async()
        .run()
        .await;

    format!(
        "successfully pinged with amount {:#?}: {:?}",
        ping_amount, response
    )
}

#[post("/pong")]
pub async fn pong(data: web::Data<AppState>, body: web::Json<PongReqBody>) -> impl Responder {
    // get a mutable lock on the contract_interact (entire struct)
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };

    let sender = body.get_tx_sending_values();

    let wallet_address = Bech32Address::from_bech32_string(sender);
    let current_address = contract_interact.state.current_address().clone();

    // access both interactor and state through the mutable borrow
    let response = contract_interact
        .interactor
        .tx()
        .from(wallet_address)
        .to(current_address)
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .pong()
        .returns(ReturnsRawResult)
        .prepare_async()
        .run()
        .await;

    format!("successfully ponged with response {:#?}", response)
}

#[post("/pong_all")]
pub async fn pong_all(data: web::Data<AppState>) -> impl Responder {
    // get a mutable lock on the contract_interact (entire struct)
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };
    let wallet_address = contract_interact.wallet_address.clone();
    let current_address = contract_interact.state.current_address().clone();

    // access both interactor and state through the mutable borrow
    let response = contract_interact
        .interactor
        .tx()
        .from(wallet_address)
        .to(current_address)
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .pong_all()
        .returns(ReturnsRawResult)
        .prepare_async()
        .run()
        .await;

    format!("successfully ponged with response {:#?}", response)
}

pub fn tx_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(ping).service(pong).service(pong_all);
}

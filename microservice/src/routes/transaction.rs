use actix_web::{post, web, Responder};
use imports::{Bech32Address, IgnoreValue, ReturnsRawResult};
use interactor::ContractInteract;
use redis::{AsyncCommands, Client};

use crate::routes::{proxy, tx_models::*};
use multiversx_sc_snippets::*;

#[post("/ping")]
pub async fn ping(body: web::Json<PingReqBody>, redis_client: web::Data<Client>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let (amount, sender) = body.get_tx_sending_values();

    let wallet_address = Bech32Address::from_bech32_string(sender);
    let current_address = contract_interact.state.current_address().clone();
    let ping_amount = amount as u64;
    let _data = IgnoreValue;

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

    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let _: () = con.del("user_addresses").await.unwrap();

    format!(
        "successfully pinged with amount {:#?}: {:?}",
        ping_amount, response
    )
}

#[post("/pong")]
pub async fn pong(body: web::Json<PongReqBody>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let sender = body.get_tx_sending_values();

    let wallet_address = Bech32Address::from_bech32_string(sender);
    let current_address = contract_interact.state.current_address().clone();

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
pub async fn pong_all() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let wallet_address = contract_interact.wallet_address.clone();
    let current_address = contract_interact.state.current_address().clone();

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

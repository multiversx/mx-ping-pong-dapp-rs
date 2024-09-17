use std::str::FromStr;

use actix_web::{post, web, Responder};
use imports::{BigUint, IgnoreValue, ReturnsRawResult};
use interactor::ContractInteract;
use multiversx_sc_snippets::imports::RustBigUint;
use redis::{AsyncCommands, Client};

use crate::routes::{proxy, tx_models::*};
use multiversx_sc_snippets::*;

#[post("/ping")]
pub async fn ping(body: web::Json<PingReqBody>, redis_client: web::Data<Client>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let amount = body.get_denominated_amount();
    let amount_numeric = RustBigUint::from_str(&amount).unwrap();

    let wallet_address = contract_interact.wallet_address.clone();
    let current_address = contract_interact.state.current_address().clone();
    let _data = IgnoreValue;

    let _response = contract_interact
        .interactor
        .tx()
        .from(wallet_address)
        .to(current_address)
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .ping(_data)
        .egld(BigUint::from(&amount_numeric))
        .returns(ReturnsRawResult)
        .prepare_async()
        .run()
        .await;

    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let _: () = con.del("user_addresses").await.unwrap();

    PingResponse::new("ok".to_string(), amount_numeric).response()
}

#[post("/pong")]
pub async fn pong() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let wallet_address = contract_interact.wallet_address.clone();
    let current_address = contract_interact.state.current_address().clone();

    let _response = contract_interact
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

    SuccessTxResponse::new("ok".to_string()).response()
}

#[post("/pong_all")]
pub async fn pong_all() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let wallet_address = contract_interact.wallet_address.clone();
    let current_address = contract_interact.state.current_address().clone();

    let _response = contract_interact
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

    SuccessTxResponse::new("ok".to_string()).response()
}

pub fn tx_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(ping).service(pong).service(pong_all);
}

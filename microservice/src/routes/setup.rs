use std::str::FromStr;

use actix_web::{get, post, Responder};
use actix_web::{web, HttpResponse};
use imports::{bech32, Bech32Address, BigUint, OptionalValue, ReturnsNewAddress, RustBigUint};
use interactor_microservice::ContractInteract;
use serde_json::json;

use crate::routes::helpers::{DeployReqBody, DeployResponse};
use crate::routes::proxy;
use multiversx_sc_snippets::*;
use redis::{AsyncCommands, Client};

#[post("")]
pub async fn setup_contract(
    body: web::Json<DeployReqBody>,
    redis_client: web::Data<Client>,
) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let (amount, max_funds, activation_timestamp, duration) = body.get_tx_sending_values();

    let contract_code = contract_interact.contract_code.clone();

    let ping_amount = RustBigUint::from_str(&amount).unwrap();
    let duration_in_seconds = duration;

    let opt_activation_timestamp = match activation_timestamp {
        0 => None,
        _ => Some(activation_timestamp),
    };

    let max_funds = OptionalValue::Some(RustBigUint::from_str(&max_funds).unwrap());

    let new_address = contract_interact
        .interactor
        .tx()
        .from(contract_interact.wallet_address)
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .init(
            BigUint::from(&ping_amount),
            duration_in_seconds,
            opt_activation_timestamp,
            max_funds,
        )
        .code(contract_code)
        .returns(ReturnsNewAddress)
        .prepare_async()
        .run()
        .await;

    let new_address_bech32 = bech32::encode(&new_address);
    contract_interact
        .state
        .set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    // Invalidate values corresponding to previous deployed contract
    let _: () = con.del("user_addresses").await.unwrap();
    let _: () = con.del("ping_amount").await.unwrap();
    let _: () = con.del("max_funds").await.unwrap();
    let _: () = con.del("deadline").await.unwrap();
    let _: () = con.del("timestamp").await.unwrap();

    DeployResponse::new(("ok".to_string(), new_address_bech32)).send()
}

#[get("/contract_address")]
pub async fn get_contract_address() -> impl Responder {
    let contract_interact = ContractInteract::new().await;

    HttpResponse::Ok()
        .json(json!({"contract_address": contract_interact.state.current_address().to_string()}))
}

pub fn setup_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(setup_contract).service(get_contract_address);
}

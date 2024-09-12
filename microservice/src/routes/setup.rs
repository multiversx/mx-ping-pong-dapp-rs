use actix_web::web;
use actix_web::{post, Responder};
use imports::{bech32, Bech32Address, OptionalValue, ReturnsNewAddress, RustBigUint};
use interactor::ContractInteract;

use crate::routes::proxy;
use crate::routes::tx_models::*;
use multiversx_sc_snippets::*;
use redis::{AsyncCommands, Client};

#[post("/setup")]
pub async fn setup_contract(
    body: web::Json<DeployReqBody>,
    redis_client: web::Data<Client>,
) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let (amount, max_funds, _activation_timestamp, duration, deployer) =
        body.get_tx_sending_values();

    let contract_code = contract_interact.contract_code.clone();
    let wallet_address = Bech32Address::from_bech32_string(deployer);

    let ping_amount = RustBigUint::from(amount);
    let duration_in_seconds = duration;
    let opt_activation_timestamp: Option<u64> = None;
    let max_funds = OptionalValue::Some(RustBigUint::from(max_funds));

    let new_address = contract_interact
        .interactor
        .tx()
        .from(wallet_address)
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .init(
            ping_amount,
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

    let _: () = con
        .set("contract_address", &new_address_bech32)
        .await
        .unwrap();

    // Invalidate values corresponding to previous deployed contract
    let _: () = con.del("user_addresses").await.unwrap();
    let _: () = con.del("ping_amount").await.unwrap();
    let _: () = con.del("max_funds").await.unwrap();
    let _: () = con.del("deadline").await.unwrap();
    let _: () = con.del("timestamp").await.unwrap();

    format!("new address: {new_address_bech32}")
}

use actix_web::{post, Responder};
use imports::{bech32, Bech32Address, OptionalValue, ReturnsNewAddress, RustBigUint};
use interactor::ContractInteract;

use crate::routes::proxy;
use multiversx_sc_snippets::*;

#[post("/setup")]
pub async fn setup_contract(
) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let contract_code = contract_interact.contract_code.clone();
    let wallet_address = contract_interact.wallet_address.clone();

    let ping_amount = RustBigUint::from(1u128);
    let duration_in_seconds = 12u64;
    let opt_activation_timestamp = Option::Some(108u64);
    let max_funds = OptionalValue::Some(RustBigUint::from(123_000u128));

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

    format!("new address: {new_address_bech32}")
}

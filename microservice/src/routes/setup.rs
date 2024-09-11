use actix_web::{post, web, Responder};
use imports::{bech32, Bech32Address, OptionalValue, ReturnsNewAddress, RustBigUint};

use crate::routes::proxy;
use crate::routes::tx_models::*;
use crate::shared_state::AppState;
use multiversx_sc_snippets::*;

#[post("/setup")]
pub async fn setup_contract(
    data: web::Data<AppState>,
    body: web::Json<DeployReqBody>,
) -> impl Responder {
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };

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

    format!("new address: {new_address_bech32}")
}

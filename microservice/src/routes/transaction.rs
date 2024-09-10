use actix_web::{post, web, Responder};
use imports::{IgnoreValue, ReturnsRawResult};

use crate::routes::proxy;
use crate::shared_state::AppState;
use multiversx_sc_snippets::*;

#[post("/transaction")]
pub async fn ping(data: web::Data<AppState>) -> impl Responder {
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
    let ping_amount = 5u64;
    let _data = IgnoreValue;

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

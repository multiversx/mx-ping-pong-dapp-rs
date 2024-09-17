use std::str::FromStr;

use imports::{BigUint, IgnoreValue, ReturnsRawResult};
use interactor_frontend::ContractInteract;
use multiversx_sc_snippets::imports::RustBigUint;

use multiversx_sc_snippets::*;

use super::{proxy, tx_models::*};

pub async fn ping_tx(body: PingReqBody) -> PingResponse {
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

    PingResponse::new("ok".to_string(), amount_numeric)
}

pub async fn pong_tx() -> SuccessTxResponse {
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

    SuccessTxResponse::new("ok".to_string())
}

pub async fn pong_all_tx() -> SuccessTxResponse {
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

    SuccessTxResponse::new("ok".to_string())
}

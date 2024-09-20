use std::str::FromStr;

use multiversx_my_sc_snippets::{
    hex,
    imports::{
        Bech32Address, BigUint, BytesValue, IgnoreValue, OptionalValue, ReturnsMessage,
        ReturnsNewAddress, ReturnsStatus, RustBigUint,
    },
    sdk::gateway::DEVNET_GATEWAY,
    test_wallets, Interactor, InteractorPrepareAsync,
};
use serde_json::{json, Value};

use crate::{
    context::ConfigContext,
    requests::{contract_code, helpers::*},
    transactions::proxy,
};

pub enum TransactionType {
    Ping(String),
    Pong,
    PongAll,
}

pub async fn ping(contract_addr: &str, amount: String) -> Result<Value, Value> {
    let ping_amount = amount.parse::<f64>().unwrap();
    let denominated_ping_amount = RustBigUint::from_str(&denominate(ping_amount)).unwrap();

    let mut interact = Interactor::new(DEVNET_GATEWAY).await;
    let addr = interact.register_wallet(test_wallets::alice());

    let (status, message) = interact
        .tx()
        .from(Bech32Address::from(addr))
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .ping(IgnoreValue)
        .to(Bech32Address::from_bech32_string(contract_addr.to_string()))
        .egld(BigUint::from(denominated_ping_amount))
        .returns(ReturnsStatus)
        .returns(ReturnsMessage)
        .prepare_async()
        .run()
        .await;

    Ok(json!({
        "status": status,
        "message": message
    }))
}

pub async fn pong(contract_addr: &str) -> Result<Value, Value> {
    let mut interactor = Interactor::new(DEVNET_GATEWAY).await;
    let addr = interactor.register_wallet(test_wallets::alice());

    let (status, message) = interactor
        .tx()
        .from(Bech32Address::from(addr))
        .to(Bech32Address::from_bech32_string(contract_addr.to_string()))
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .pong()
        .returns(ReturnsStatus)
        .returns(ReturnsMessage)
        .prepare_async()
        .run()
        .await;

    Ok(json!({
        "status": status,
        "message": message,
    }))
}

pub async fn pong_all(contract_addr: &str) -> Result<Value, Value> {
    let mut interactor = Interactor::new(DEVNET_GATEWAY).await;
    let addr = interactor.register_wallet(test_wallets::alice());

    let (status, message) = interactor
        .tx()
        .from(Bech32Address::from(addr))
        .to(Bech32Address::from_bech32_string(contract_addr.to_string()))
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .pong_all()
        .returns(ReturnsStatus)
        .returns(ReturnsMessage)
        .prepare_async()
        .run()
        .await;

    Ok(json!({
        "status": status,
        "message": message,
    }))
}

pub async fn sc_setup(
    context: &ConfigContext,
    ping_amount: String,
    max_funds: String,
    activation_timestamp: String,
    duration: String,
) -> Result<Value, Value> {
    let contract_code = BytesValue::from(hex::decode(contract_code::MXSC_FILE_CONTENT).unwrap());

    let ping_amount = ping_amount.parse::<f64>().unwrap();
    let max_funds = max_funds.parse::<f64>().unwrap();
    let duration = duration.parse::<u64>().unwrap();
    let activation_timestamp = activation_timestamp.parse::<u64>().unwrap();

    let opt_activation_timestamp = match activation_timestamp {
        0 => None,
        _ => Some(activation_timestamp),
    };
    let denominated_ping_amount = RustBigUint::from_str(&denominate(ping_amount)).unwrap();
    let denominated_max_funds = RustBigUint::from_str(&denominate(max_funds)).unwrap();

    let mut interactor = Interactor::new(DEVNET_GATEWAY).await;
    let addr = interactor.register_wallet(test_wallets::alice());

    let (new_address, status, message) = interactor
        .tx()
        .from(addr)
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .init(
            denominated_ping_amount,
            duration,
            opt_activation_timestamp,
            OptionalValue::Some(denominated_max_funds),
        )
        .code(contract_code)
        .returns(ReturnsNewAddress)
        .returns(ReturnsStatus)
        .returns(ReturnsMessage)
        .prepare_async()
        .run()
        .await;

    let addr_str = Bech32Address::from(new_address.clone()).to_string();

    context
        .set_contract_address
        .emit(Bech32Address::from(new_address).to_string());

    Ok(json!({
        "status": status,
        "message": message,
        "contract_address": addr_str,
    }))
}

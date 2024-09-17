use std::str::FromStr;

use imports::{Address, BigUint, IgnoreValue, ReturnsRawResult};
use interactor_frontend::ContractInteract;
use multiversx_sc_snippets::imports::RustBigUint;

use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

use multiversx_sc_snippets::*;

use super::{proxy, tx_models::*};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub nonce: u64,
    pub value: String,
    pub receiver: Address,
    pub sender: Address,
    pub gas_price: u64,
    pub gas_limit: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "chainID")]
    pub chain_id: String,
    pub version: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub options: u32,
}

impl Transaction {
    pub fn new(
        nonce: u64,
        value: String,
        receiver: Address,
        sender: Address,
        gas_price: u64,
        gas_limit: u64,
    ) -> Self {
        Self {
            nonce,
            value,
            receiver,
            sender,
            gas_price,
            gas_limit,
            data: Some("ping".to_string()),
            signature: None,
            chain_id: "".to_string(),
            version: 2,
            options: 2,
        }
    }

    pub fn add_sig(&mut self, signature: String) {
        self.signature = Some(signature)
    }
}
/*
}
#[wasm_bindgen]
pub async fn tx_request(endpoint: &str, body: &str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(body));

    let headers = Headers::new().unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);

    let url = format!("http://localhost:8000/tx/{}", endpoint);
    let req = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&req)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}
*/

/*
pub async fn send_transaction(&self, tx: &Transaction) -> Result<String> {
        let endpoint = self.get_endpoint(SEND_TRANSACTION_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(tx)
            .send()
            .await?
            .json::<SendTransactionResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.tx_hash),
        }
    }
*/

pub async fn ping_tx(body: PingReqBody) -> PingResponse {
    let mut contract_interact = ContractInteract::new().await;
    let amount = body.get_denominated_amount();
    let amount_numeric = RustBigUint::from_str(&amount).unwrap();

    let wallet_address = contract_interact.wallet_address.clone();
    let current_address = contract_interact.state.current_address().clone();
    let _data = IgnoreValue;

    let tx = Transaction::new(9133, "1000000000000000".to_string(), Address::from("erd1qqqqqqqqqqqqqpgq8eejxmq9z4xttxjae95a7wxv43p4r90pd8ss4fshxv".to_string()), Address::from("erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th".to_string()), 50000000000000, 30000000);

    tx.add_sig(r#"{"address":"erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th","message":"0x70696e67","signature":"0xcaf6478b44cbbb2b2e4ab0065335e341f6663adc2d565ad844aee3d5ddd5078bdc8f8d84ac5b4e6d504f012ee8e979dafb12a9aad0af62977907d5906410ba0f","version":1,"signer":"ErdJS"}"#.to_string());

    let body = json!(tx);

    // let opts = RequestInit::new();
    // opts.set_method("POST");
    // opts.set_mode(RequestMode::Cors);
    // opts.set_body(&JsValue::from_str(body));

    // let headers = Headers::new().unwrap();
    // headers.set("Content-Type", "application/json").unwrap();
    // opts.set_headers(&headers);

    // let url = format!("https://devnet-gateway.multiversx.com/transaction/send");
    // let req = Request::new_with_str_and_init(&url, &opts).unwrap();

    // let window = web_sys::window().unwrap();

    // let resp_value = JsFuture::from(window.fetch_with_request(&req)).await?;

    // assert!(resp_value.is_instance_of::<Response>());
    // let resp: Response = resp_value.dyn_into().unwrap();

    // let json = JsFuture::from(resp.json()?).await?;

    PingResponse::new("ok".to_string(), amount_numeric)
}

pub async fn pong_tx() -> SuccessTxResponse {
    let mut contract_interact = ContractInteract::new().await;

    let wallet_address = contract_interact.wallet_address.clone();
    let current_address = contract_interact.state.current_address().clone();

    // let _response = contract_interact
    //     .interactor
    //     .tx()
    //     .from(wallet_address)
    //     .to(current_address)
    //     .gas(30_000_000u64)
    //     .typed(proxy::PingPongProxy)
    //     .pong()
    //     .returns(ReturnsRawResult)
    //     .prepare_async()
    //     .run()
    //     .await;

    SuccessTxResponse::new("ok".to_string())
}

pub async fn pong_all_tx() -> SuccessTxResponse {
    let mut contract_interact = ContractInteract::new().await;
    let wallet_address = contract_interact.wallet_address.clone();
    let current_address = contract_interact.state.current_address().clone();

    // let _response = contract_interact
    //     .interactor
    //     .tx()
    //     .from(wallet_address)
    //     .to(current_address)
    //     .gas(30_000_000u64)
    //     .typed(proxy::PingPongProxy)
    //     .pong_all()
    //     .returns(ReturnsRawResult)
    //     .prepare_async()
    //     .run()
    //     .await;

    SuccessTxResponse::new("ok".to_string())
}

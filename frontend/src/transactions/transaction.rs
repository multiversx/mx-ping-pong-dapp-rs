use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode};

use super::tx_models::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub async fn ping_tx() -> SuccessTxResponse {
    // 0.001 EGLD
    // ping

    let emitted_tx = r#"{
        "nonce": 26880,
        "value": "1000000000000000",
        "receiver": "erd1qqqqqqqqqqqqqpgq8eejxmq9z4xttxjae95a7wxv43p4r90pd8ss4fshxv",
        "sender": "erd1tjkfemhpxmch4vx306y85x2lv2n9d6hvn8qpe6atc7m82wef75pqmnws0t",
        "gasPrice": 1000000000,
        "gasLimit": 30000000,
        "chainID": "D",
        "senderUsername": "",
        "receiverUsername": "",
        "data": "cGluZw==",
        "version": 2,
        "options": 0,
        "guardian": "",
        "signature": "687920cdf1416718a6bd78bdabcf2a1e1c1a443c1c75e849522932ffa88b667050254ea3567bd35481b05714f187a01990c776ef9b4dc8c7a0540899bad5aa0a",
        "guardianSignature": ""
    }"#;

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(emitted_tx));

    let headers = Headers::new().unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);

    let url = "https://devnet-gateway.multiversx.com/transaction/send".to_string();
    let req = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();

    let _resp_value = JsFuture::from(window.fetch_with_request(&req)).await;

    SuccessTxResponse::new("ok".to_string())
}

pub async fn _pong_tx() -> SuccessTxResponse {
    SuccessTxResponse::new("ok".to_string())
}

pub async fn _pong_all_tx() -> SuccessTxResponse {
    SuccessTxResponse::new("ok".to_string())
}

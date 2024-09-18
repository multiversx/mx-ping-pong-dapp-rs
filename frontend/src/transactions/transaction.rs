use std::str::FromStr;

// use imports::{Address, BigUint, IgnoreValue, ReturnsRawResult};
// use interactor_frontend::ContractInteract;
// use multiversx_sc_snippets::imports::RustBigUint;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::JSON, Headers, Request, RequestInit, RequestMode, Response};

// use multiversx_sc_snippets::*;

use super::tx_models::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Transaction {
//     pub nonce: u64,
//     pub value: String,
//     pub receiver: Address,
//     pub sender: Address,
//     pub gas_price: u64,
//     pub gas_limit: u64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub data: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub signature: Option<String>,
//     #[serde(rename = "chainID")]
//     pub chain_id: String,
//     pub version: u32,
//     #[serde(skip_serializing_if = "is_zero")]
//     pub options: u32,
// }

// impl Transaction {
//     pub fn new(
//         nonce: u64,
//         value: String,
//         receiver: Address,
//         sender: Address,
//         gas_price: u64,
//         gas_limit: u64,
//     ) -> Self {
//         Self {
//             nonce,
//             value,
//             receiver,
//             sender,
//             gas_price,
//             gas_limit,
//             data: Some("ping".to_string()),
//             signature: None,
//             chain_id: "".to_string(),
//             version: 2,
//             options: 2,
//         }
//     }

//     pub fn add_sig(&mut self, signature: String) {
//         self.signature = Some(signature)
//     }
// }
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


/*
{
    "emittedTransaction": {
        "nonce": 3903,
        "value": "0",
        "receiver": "erd1qqqqqqqqqqqqqpgq45zs77q884ts6y9zj4jyqfn6ydev8ruv2jps3tteqq",
        "sender": "erd1ss6u80ruas2phpmr82r42xnkd6rxy40g9jl69frppl4qez9w2jpsqj8x97",
        "gasPrice": 1000000000,
        "gasLimit": 100000000,
        "chainID": "1",
        "senderUsername": "",
        "receiverUsername": "",
        "data": "d2l0aGRyYXdSZXdhcmRzQDAx",
        "version": 1,
        "options": 0,
        "guardian": "",
        "signature": "d3f389da5e5077cce3304f08698c90ae09418b9bff7a1803003b60b0a475701e99326938fc391e32a94d81438f4d103c7700082c3158987401e5c7c75c8a5708",
        "guardianSignature": ""
    },
    "emittedTransactionData": "withdrawRewards@01",
    "emittedTransactionHash": "5ae26c8f6a9b917f628f5f49232c739f6bff021624dc16bd26a39cea70c488fb"
}
*/
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

    // let tx_json = r#"{"nonce":7752,"value":"1000000000000000","receiver":"erd1qqqqqqqqqqqqqpgq8eejxmq9z4xttxjae95a7wxv43p4r90pd8ss4fshxv","sender":"erd1tjkfemhpxmch4vx306y85x2lv2n9d6hvn8qpe6atc7m82wef75pqmnws0t","gasPrice":1000000000,"gasLimit":30000000,"data":"cGluZw==","chainID":"1","version":1,"signature":"{"address":"erd1tjkfemhpxmch4vx306y85x2lv2n9d6hvn8qpe6atc7m82wef75pqmnws0t","message":"0x7b226e6f6e6365223a373735322c2276616c7565223a2231303030303030303030303030303030222c227265636569766572223a2265726431717171717171717171717171717067713865656a786d71397a34787474786a616539356137777876343370347239307064387373346673687876222c2273656e646572223a2265726431746a6b66656d6870786d636834767833303679383578326c76326e39643668766e3871706536617463376d3832776566373570716d6e77733074222c226761735072696365223a313030303030303030302c226761734c696d6974223a33303030303030302c2264617461223a2263476c755a773d3d222c22636861696e4944223a2231222c2276657273696f6e223a317d","signature":"0x85061c1588044a285dab4d224876fe24c5df58b2096423c027f09ce8dbae0dc3f7b07f4ae465a1e5642370f14051a17d6bf16d27310022dfda58bb9d6bfb7d0a","version":1,"signer":"ErdJS"}"#;

    // let js_body_value: JsValue = JsValue::from_str(tx_json);
    // log::info!("I'M HERE {:?}", js_body_value.clone());

    // let js_string = JSON::stringify(&js_body_value).unwrap();
    // log::info!("{}", &js_string);

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&emitted_tx));

    let headers = Headers::new().unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);

    let url = format!("https://devnet-gateway.multiversx.com/transaction/send");
    let req = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&req)).await;

    SuccessTxResponse::new("ok".to_string())
}

/*
# With data field
nonce = 7
value = "10000000000000000000"  # 10 EGLD
receiver = "erd1cux02zersde0l7hhklzhywcxk4u9n4py5tdxyx7vrvhnza2r4gmq4vw35r"
sender = "erd1l453hd0gt5gzdp7czpuall8ggt2dcv5zwmfdf3sd3lguxseux2fsmsgldz"
gasPrice = 1000000000
gasLimit = 70000
data = "Zm9yIHRoZSBib29r"
chainID = "1"
version = 1
signature = "1702bb7696f992525fb77597956dd74059b5b01e88c813066ad1f6053c6afca97d6eaf7039b2a21cccc7d73b3e5959be4f4c16f862438c7d61a30c91e3d16c01"
*/

pub async fn pong_tx() -> SuccessTxResponse {
    // let mut contract_interact = ContractInteract::new().await;

    // let wallet_address = contract_interact.wallet_address.clone();
    // let current_address = contract_interact.state.current_address().clone();

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
    // let mut contract_interact = ContractInteract::new().await;
    // let wallet_address = contract_interact.wallet_address.clone();
    // let current_address = contract_interact.state.current_address().clone();

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

use crate::data::{
    address::Address,
    network_config::NetworkConfig,
    transaction::{
        ArgCreateTransaction, ResponseTxCost, SendTransactionResponse, SendTransactionsResponse,
        Transaction, TransactionInfo, TransactionOnNetwork, TransactionStatus, TxCostResponseData,
    },
    vm::{ResponseVmValue, VmValueRequest, VmValuesResponseData},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use serde_json::to_string_pretty;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode};

use super::GatewayProxy;

const COST_TRANSACTION_ENDPOINT: &str = "transaction/cost";
const SEND_TRANSACTION_ENDPOINT: &str = "transaction/send";
const SEND_MULTIPLE_TRANSACTIONS_ENDPOINT: &str = "transaction/send-multiple";
const GET_TRANSACTION_INFO_ENDPOINT: &str = "transaction/";
const WITH_RESULTS_QUERY_PARAM: &str = "?withResults=true";
const VM_VALUES_ENDPOINT: &str = "vm-values/query";

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

impl GatewayProxy {
    // request_transaction_cost retrieves how many gas a transaction will consume
    pub async fn request_transaction_cost(&self, tx: &Transaction) -> Result<TxCostResponseData> {
        let endpoint = self.get_endpoint(COST_TRANSACTION_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(tx)
            .send()
            .await?
            .json::<ResponseTxCost>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b),
        }
    }

    async fn get_transaction_info_internal(
        &self,
        hash: &str,
        with_results: bool,
    ) -> Result<TransactionOnNetwork> {
        let mut endpoint = GET_TRANSACTION_INFO_ENDPOINT.to_string() + hash;

        if with_results {
            endpoint += WITH_RESULTS_QUERY_PARAM
        }

        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<TransactionInfo>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.transaction),
        }
    }

    // get_transaction_info retrieves a transaction's details from the network
    pub async fn get_transaction_info(&self, hash: &str) -> Result<TransactionOnNetwork> {
        self.get_transaction_info_internal(hash, false).await
    }

    // get_transaction_info_with_results retrieves a transaction's details from the network with events
    pub async fn get_transaction_info_with_results(
        &self,
        hash: &str,
    ) -> Result<TransactionOnNetwork> {
        self.get_transaction_info_internal(hash, true).await
    }

    // get_transaction_status retrieves a transaction's status from the network
    pub async fn get_transaction_status(&self, hash: &str) -> Result<String> {
        let endpoint = format!("transaction/{hash}/status");
        let endpoint = self.get_endpoint(endpoint.as_str());

        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<TransactionStatus>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.status),
        }
    }

    // get_default_transaction_arguments will prepare the transaction creation argument by querying the account's info
    pub async fn get_default_transaction_arguments(
        &self,
        address: &Address,
        network_configs: &NetworkConfig,
    ) -> Result<ArgCreateTransaction> {
        let account = self.get_account(address).await?;

        Ok(ArgCreateTransaction {
            nonce: account.nonce,
            value: "".to_string(),
            rcv_addr: address.clone(),
            snd_addr: address.clone(),
            gas_price: network_configs.min_gas_price,
            gas_limit: network_configs.min_gas_limit,
            data: None,
            signature: "".to_string(),
            chain_id: network_configs.chain_id.clone(),
            version: network_configs.min_transaction_version,
            options: 0,
            available_balance: account.balance,
        })
    }

    pub fn send_transaction(&self, tx: &Transaction) -> String {
        let tx_str = to_string_pretty(&tx).unwrap();
        let endpoint = self.get_endpoint(SEND_TRANSACTION_ENDPOINT);

        let window = web_sys::window().unwrap();
        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::Cors);
        opts.set_body(&JsValue::from_str(&tx_str));

        let request = Request::new_with_str_and_init(&endpoint, &opts).unwrap();
        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();

        let fetch_promise = window.fetch_with_request(&request);

        let on_success = Closure::wrap(Box::new(move |_response: JsValue| {
            alert("Transaction sent successfully!");
        }) as Box<dyn FnMut(JsValue)>);

        let on_error = Closure::wrap(Box::new(move |_err: JsValue| {
            alert("Failed to send transaction!");
        }) as Box<dyn FnMut(JsValue)>);

        fetch_promise.then(&on_success).catch(&on_error);

        on_success.forget();
        on_error.forget();

        "Ok".to_string()
    }

    pub async fn send_transactions(&self, txs: &Vec<Transaction>) -> Result<Vec<String>> {
        let endpoint = self.get_endpoint(SEND_MULTIPLE_TRANSACTIONS_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(txs)
            .send()
            .await?
            .json::<SendTransactionsResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => {
                let mut tx_hashs: Vec<String> = vec![];
                for key in b.txs_hashes.keys().sorted() {
                    tx_hashs.push(b.txs_hashes[key].clone());
                }

                Ok(tx_hashs)
            }
        }
    }

    // execute_vmquery retrieves data from existing SC trie through the use of a VM
    pub async fn execute_vmquery(
        &self,
        vm_request: &VmValueRequest,
    ) -> Result<VmValuesResponseData> {
        let endpoint = self.get_endpoint(VM_VALUES_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(vm_request)
            .send()
            .await?
            .json::<ResponseVmValue>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b),
        }
    }
}

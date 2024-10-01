use std::{env, io::Write};

use imports::{Address, Bech32Address, BytesValue, InterpretableFrom, InterpreterContext};
use multiversx_sc_snippets::*;
use serde::{Deserialize, Serialize};

const GATEWAY: &str = sdk::gateway::DEVNET_GATEWAY;
const ENV_FILE: &str = ".env";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if let Ok(contract_address_str) = env::var("CONTRACT_ADDRESS") {
            Self {
                contract_address: Some(Bech32Address::from_bech32_string(contract_address_str)),
            }
        } else {
            println!("No contract address found in env");
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        if let Some(address) = &self.contract_address {
            update_env_variable(ENV_FILE, "CONTRACT_ADDRESS", address.to_bech32_str());
        }
    }
}

fn update_env_variable(file_path: &str, key: &str, new_value: &str) {
    let content = std::fs::read_to_string(file_path).unwrap();

    let updated_content = content
        .lines()
        .map(|line| {
            if line.starts_with(key) {
                format!("{}={}", key, new_value)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(updated_content.as_bytes()).unwrap();
}

pub struct ContractInteract {
    pub interactor: Interactor,
    pub wallet_address: Address,
    pub contract_code: BytesValue,
    pub state: State,
}

impl ContractInteract {
    pub async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(test_wallets::alice());

        let contract_code = BytesValue::interpret_from(
            "mxsc:microservice/ping-pong-egld.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }
}

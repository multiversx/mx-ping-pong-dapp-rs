use actix::{
    prelude::{Actor, Context},
    AsyncContext,
};
use tokio::time::Duration;
use types::ContractState;

use dotenv::dotenv;
use imports::{Bech32Address, ReturnsResultUnmanaged};
use multiversx_sc_snippets::*;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::env;

mod proxy;
mod types;

const GATEWAY: &str = sdk::gateway::DEVNET_GATEWAY;

struct Scheduler {
    con: MultiplexedConnection,
    contract_address: String,
}

impl Actor for Scheduler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // schedule the first job
        ctx.run_interval(Duration::from_secs(60), |this, _| {
            // spawn a new task for the job
            let mut con = this.con.clone();
            let contract_address = this.contract_address.clone();
            actix_rt::spawn(async move {
                if let Err(e) = fetch_and_update_contract_state(&mut con, contract_address).await {
                    println!("Error during cron job: {:?}", e);
                }
            });
        });
    }
}

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    let redis_client =
        Client::open(env::var("REDIS_URL").unwrap()).expect("Failed to connect to Redis server");

    let con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();
    let contract_address = env::var("CONTRACT_ADDRESS").unwrap();

    // start the system and the scheduler actor
    Scheduler {
        con,
        contract_address,
    }
    .start();

    // keep the system running indefinitely
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

async fn fetch_and_update_contract_state(
    con: &mut MultiplexedConnection,
    contract_address: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract_state = prepare_contract_state_query(contract_address).await;

    let serialized_response = serde_json::to_string(&contract_state).unwrap();

    let _: () = con
        .set("contract_state", serialized_response)
        .await
        .unwrap();

    println!("Updated Redis with contract state: {:#?}", contract_state);

    Ok(())
}

async fn prepare_contract_state_query(contract_address: String) -> ContractState {
    let mut contract_interact = Interactor::new(GATEWAY).await;

    let result_value = contract_interact
        .query()
        .to(Bech32Address::from_bech32_string(contract_address))
        .typed(proxy::PingPongProxy)
        .get_contract_state()
        .returns(ReturnsResultUnmanaged)
        .prepare_async()
        .run()
        .await;

    ContractState::from(result_value)
}

use actix_web::{get, web, Responder};
use imports::ReturnsResultUnmanaged;

use crate::routes::proxy;
use crate::shared_state::AppState;
use multiversx_sc_snippets::*;

#[get("/query")]
pub async fn get_deadline(data: web::Data<AppState>) -> impl Responder {
    // get a mutable lock on the contract_interact (entire struct)
    let mut contract_interact = match data.interactor.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            // log the error
            return format!("Failed to acquire lock: {:?}", poisoned);
        }
    };
    let current_address = contract_interact.state.current_address().clone();

    // access both interactor and state through the mutable borrow
    let result_value = contract_interact
        .interactor
        .query()
        .to(current_address) // access state via the mutable reference
        .typed(proxy::PingPongProxy)
        .deadline()
        .returns(ReturnsResultUnmanaged)
        .prepare_async()
        .run()
        .await;

    format!("Result: {result_value:?}")
}

use actix_web::{web, App, HttpServer};
use std::sync::{Arc, RwLock};
mod shared_state;
pub use shared_state::AppState;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let interactor = interactor::ContractInteract::new().await;
    let shared_interactor = Arc::new(RwLock::new(interactor));

    // start the Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                interactor: shared_interactor.clone(),
            }))
            .service(routes::query::get_deadline)
            .service(routes::setup::setup_contract)
            .service(routes::transaction::ping)
    })
    .bind("127.0.0.1:8084")?
    .run()
    .await
}

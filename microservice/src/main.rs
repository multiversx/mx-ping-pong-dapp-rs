use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use routes::{query_configuration, tx_configuration};
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
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .service(routes::query::get_deadline)
            .service(routes::setup::setup_contract)
            .service(web::scope("/query").configure(query_configuration))
            .service(web::scope("/tx").configure(tx_configuration))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

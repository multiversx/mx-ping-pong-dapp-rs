use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use redis::{AsyncCommands, Client};
use routes::{query_configuration, tx_configuration};
use std::env;
use std::sync::{Arc, RwLock};
mod shared_state;
pub use shared_state::AppState;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let interactor = interactor::ContractInteract::new().await;
    let shared_interactor = Arc::new(RwLock::new(interactor));

    let redis_client =
        Client::open(env::var("REDIS_URL").unwrap()).expect("Failed to connect to Redis server");

    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let _: () = redis::cmd("FLUSHALL")
        .query_async(&mut con)
        .await
        .expect("Failed to flush Redis");

    // start the Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                interactor: shared_interactor.clone(),
            }))
            .app_data(web::Data::new(redis_client.clone()))
            .service(routes::setup::setup_contract)
            .service(web::scope("/query").configure(query_configuration))
            .service(web::scope("/tx").configure(tx_configuration))
    })
    .bind("127.0.0.1:8084")?
    .run()
    .await
}

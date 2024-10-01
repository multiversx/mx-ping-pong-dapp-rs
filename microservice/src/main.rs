use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use redis::Client;
use routes::{query_configuration, setup_configuration, tx_configuration};
use std::env;

mod helpers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

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
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .app_data(web::Data::new(redis_client.clone()))
            .service(web::scope("/setup").configure(setup_configuration))
            .service(web::scope("/query").configure(query_configuration))
            .service(web::scope("/tx").configure(tx_configuration))
    })
    .bind("127.0.0.1:8089")?
    .run()
    .await
}

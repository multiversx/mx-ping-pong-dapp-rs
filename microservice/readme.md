# Microservice example in Rust on the Ping-Pong contract

This is a minimal microservice written in Rust that does 3 actions:
- setup the smart contract
- query the smart contract
- send a transaction to the smart contract

The microservice uses the `actix_web` library for standard server-side logic and the [`multiversx interactor`](https://docs.multiversx.com/developers/meta/interactor/interactors-overview/) for SC interaction. 

## Try this example locally

In order to run the program, clone the repository and run it with `cargo run`.
Now, the 3 routes should be available on `localhost`:
```
#[GET] - localhost::8084/query 
#[POST] - localhost::8084/setup
#[POST] - localhost::8084/transaction
```

TODO: 
- more routes per action
- caching
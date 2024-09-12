# Frontend example in Rust on the Ping-Pong contract

This is a minimal frontend project written in Rust using the `yew` library. 
The `trunk.toml` file contains deploy configs, while `config.rs` contains API configs. 
The program sends requests to the microservice endpoints (query, transactions) and renders the results.

## Prerequisites

- trunk
```bash
cargo install trunk
```
- WASM build target
```bash
rustup target add wasm32-unknown-unknown
```

## Try this example locally

In order to run the program, clone the repository and start the server with `trunk serve --open`.

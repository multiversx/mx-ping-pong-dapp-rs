# Rust dApp example for the Ping-Pong contract

The repo contains 2 projects, the microservice and the frontend, both written in Rust.

The microservice:
- queries the SC
- sends transactions to the SC
- caches results
- does the SC setup

The frontend:
- sends requests to the microservice for results
- caches values from responses for faster loading
- displays results

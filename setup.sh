#!/bin/bash

cleanup() {
    echo "Stopping all processes..."
    kill $(jobs -p)
    exit 0
}

trap cleanup SIGINT

BASE_DIR=$(pwd)

cargo run &

cd $BASE_DIR/frontend/www
npm install

cd $BASE_DIR/frontend/src
wasm-pack build

cd $BASE_DIR/frontend/www
npm run start &

wait
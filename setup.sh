#!/bin/bash

PORT_MICROSERVICE=8088
PORT_FRONTEND=8080
PORT_REDIS=6379

BASE_DIR=$(pwd)

cleanup() {
    echo "Stopping all processes..."

    kill $(jobs -p) 2>/dev/null

    wait

    exit 0
}

trap cleanup SIGINT SIGTERM EXIT

redis-server &

cargo run --bin microservice &
MICROSERVICE_PID=$!

if [ $? -ne 0 ]; then
    echo "Failed to start the microservice."
    exit 1
fi

cd "$BASE_DIR/frontend"

trunk serve --open &
FRONTEND_PID=$!

if [ $? -ne 0 ]; then
    echo "Failed to start trunk."
    kill $MICROSERVICE_PID
    exit 1
fi

wait

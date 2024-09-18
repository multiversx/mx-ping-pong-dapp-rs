cleanup() {
    echo "Stopping all processes..."
    kill $(jobs -p) 2>/dev/null
    exit 0
}

trap cleanup SIGINT SIGTERM
trap cleanup EXIT

BASE_DIR=$(pwd)

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

wait $MICROSERVICE_PID
wait $FRONTEND_PID
cleanup() {
    echo "Stopping all processes..."
    
    # Try to terminate all processes with SIGTERM
    kill $MICROSERVICE_PID $FRONTEND_PID $REDIS_PID $CRONJOB_PID 2>/dev/null
    
    echo "All processes stopped."
    exit 0
}

trap cleanup SIGINT SIGTERM SIGTSTP
trap cleanup EXIT

BASE_DIR=$(pwd)

# Start Redis
redis-server &
REDIS_PID=$!

# Check if Redis started successfully
if ! kill -0 $REDIS_PID 2>/dev/null; then
    echo "Failed to start Redis."
    exit 1
fi

# Start cron-job
cargo run --bin cron-job &
CRONJOB_PID=$!

# Check if cron-job started successfully
if ! kill -0 $CRONJOB_PID 2>/dev/null; then
    echo "Failed to start cron-job."
    exit 1
fi

# Start microservice
cargo run --bin microservice &
MICROSERVICE_PID=$!

# Check if microservice started successfully
if ! kill -0 $MICROSERVICE_PID 2>/dev/null; then
    echo "Failed to start the microservice."
    kill $REDIS_PID
    exit 1
fi

# Start frontend
cd "$BASE_DIR/frontend"
trunk serve --open &
FRONTEND_PID=$!

# Check if frontend started successfully
if ! kill -0 $FRONTEND_PID 2>/dev/null; then
    echo "Failed to start trunk."
    kill $MICROSERVICE_PID
    kill $REDIS_PID
    kill $CRONJOB_PID
    exit 1
fi

# Wait for all processes to finish
wait $MICROSERVICE_PID
wait $FRONTEND_PID
wait $REDIS_PID
wait $CRONJOB_PID
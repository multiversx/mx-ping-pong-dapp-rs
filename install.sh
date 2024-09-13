#!/bin/bash

BASE_DIR=$(pwd)

echo "Installing Redis server..."
sudo apt-get install redis-server -y

echo "Redis installation completed!"
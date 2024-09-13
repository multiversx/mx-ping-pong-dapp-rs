#!/bin/bash

# If an error is encountered -> exit
set -e
BASE_DIR=$(pwd)

print_info() {
    echo -e "\e[32m$1\e[0m"
}

print_error() {
    echo -e "\e[31m$1\e[0m" >&2
}

if [ "$EUID" -ne 0 ]; then
    print_error "Please run with sudo"
    exit 1
fi

# Install if not already installed

if ! command -v redis-server > /dev/null; then
    sudo apt-get install redis-server -y
else
    print_info "Redis is already installed."
fi

print_info "Setup completed successfully!"

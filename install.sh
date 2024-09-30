#!/bin/bash

# Build the Rust project
cargo build --release

# Copy the binary to /usr/local/bin
sudo cp target/release/lv_manager /usr/local/bin/lv_manager

echo "lv_manager installed successfully!"

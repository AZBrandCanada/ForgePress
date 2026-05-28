#!/bin/bash
set -e

echo "=========================================================="
echo "            Starting ForgePress Dev Environment           "
echo "=========================================================="

# 1. Ensure the environment configuration file exists
if [ ! -f .env ]; then
    echo "No .env file detected. Copying .env.example to .env..."
    cp .env.example .env
fi

# 2. Trap Exit Signals to Clean Up Processes on Ctrl+C
cleanup() {
    echo ""
    echo "Shutting down ForgePress developer services..."
    if [ -n "$VITE_PID" ]; then
        kill "$VITE_PID" 2>/dev/null || true
    fi
    exit 0
}
trap cleanup SIGINT SIGTERM EXIT

# 3. Start the Vite/Svelte Frontend Dev Server in the background
echo "Starting Svelte Admin Dashboard (Vite Dev Server)..."
cd admin-dashboard
# Starts Vite and redirects its logs slightly so it doesn't completely flood your terminal
npm run dev &
VITE_PID=$!
cd ..

# Give Vite 1.5 seconds to bind to its port before launching Rust
sleep 1.5

# 4. Start the Rust Core Web Server
echo "Starting Rust Core Web Server (Cargo Run)..."
cargo run --package forgepress-core
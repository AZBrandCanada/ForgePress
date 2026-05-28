#!/bin/bash
set -e
#rustup target add wasm32-wasip2
echo "=========================================================="
echo "          ForgePress Compilation & Build Script           "
echo "=========================================================="

# 1. Compile Svelte Admin Dashboard (Must occur first to allow Rust to embed it)
echo "Step 1: Checking and compiling the Admin Dashboard..."
if command -v npm &> /dev/null; then
    cd admin-dashboard
    echo "Installing frontend dependencies..."
    npm install
    echo "Compiling production frontend bundle..."
    npm run build
    cd ..
    echo "Frontend successfully compiled to: /admin-dashboard/dist"
else
    echo "Warning: Node.js/NPM is not installed."
    echo "Creating empty placeholder dist directory to prevent Rust compile-time failures."
    mkdir -p admin-dashboard/dist
    touch admin-dashboard/dist/.keep
fi

# 2. Compile the Rust Workspace
echo "Step 2: Compiling Rust workspace members in release mode..."
cargo build --release

echo "=========================================================="
echo " Success! All ForgePress components compiled successfully.  "
echo " Binaries available at:                                   "
echo "   - Core Engine: ./target/release/forgepress-core        "
echo "   - CLI Tool:    ./target/release/forgepress-cli         "
echo "=========================================================="
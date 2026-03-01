#!/bin/bash

# Helix 3D Renderer - Dependency Check Script

set -e

echo "📦 Checking dependencies..."

# Check Rust
cargo --version > /dev/null 2>&1 || { echo "❌ Cargo not found"; exit 1; }

# Check for updates (optional, non-blocking)
if command -v cargo-outdated &> /dev/null; then
    echo "📦 Checking for outdated dependencies..."
    cargo outdated || true
fi

echo "✅ Dependencies OK"

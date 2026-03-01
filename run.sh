#!/bin/bash

# Helix 3D Renderer - Unified Run Script
# Matches Helix2000 command conventions
# Usage: ./run.sh [command]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warn() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

log_section() {
    echo -e "${CYAN}\n🎯 $1${NC}"
}

# Show help
show_help() {
    echo -e "${CYAN}🎮 Helix 3D Renderer - Command Interface${NC}"
    echo ""
    echo -e "${GREEN}🚀 Quick Start Commands:${NC}"
    echo "  ./run.sh dev          Start with full checks"
    echo "  ./run.sh dev:fast     Fastest startup (skip checks)"
    echo "  ./run.sh dev:safe     Kill ports & start fresh"
    echo "  ./run.sh dev:release  Start optimized release build"
    echo "  ./run.sh dev:wsl:gpu  Start with custom Mesa GPU acceleration (WSL2)"
    echo "  ./run.sh dev:wsl:gpu:fast WSL2 GPU accelerated + release mode"
    echo ""
    echo -e "${GREEN}🧪 Testing Commands:${NC}"
    echo "  ./run.sh test         Run all tests"
    echo "  ./run.sh test:unit    Unit tests only"
    echo "  ./run.sh test:coverage Coverage report"
    echo ""
    echo -e "${GREEN}✅ Quality Commands:${NC}"
    echo "  ./run.sh quality-check Full quality pipeline"
    echo "  ./run.sh lint         Run clippy"
    echo "  ./run.sh lint:fix     Auto-fix clippy issues"
    echo "  ./run.sh format       Check formatting"
    echo "  ./run.sh format:fix   Fix formatting"
    echo ""
    echo -e "${GREEN}🔨 Build Commands:${NC}"
    echo "  ./run.sh build        Debug build"
    echo "  ./run.sh build:all    Build all targets"
    echo "  ./run.sh build:release Release build"
    echo ""
    echo -e "${GREEN}🧹 Utility Commands:${NC}"
    echo "  ./run.sh clean        Clean build artifacts"
    echo "  ./run.sh setup        Setup environment"
    echo "  ./run.sh stats        Project statistics"
    echo "  ./run.sh health-check Health check"
    echo "  ./run.sh check-gpu    Check GPU acceleration status"
    echo "  ./run.sh help         Show this help"
}

# Command handlers
cmd_dev() {
    log_section "Starting development mode with checks..."
    ./scripts/check-deps.sh 2>/dev/null || log_warn "check-deps.sh not found"
    cargo check
    cargo run --features egui
}

cmd_dev_fast() {
    log_section "Starting development mode (fast, skip checks)..."
    cargo run --features egui
}

cmd_dev_safe() {
    log_section "Safe dev mode - cleaning up first..."
    cargo clean
    ./scripts/kill-ports.sh 2>/dev/null || log_warn "kill-ports.sh not found"
    cargo check
    cargo run --features egui
}

cmd_dev_release() {
    log_section "Starting optimized release build..."
    cargo run --release --features egui
}

cmd_dev_wsl_gpu() {
    log_section "Starting with WSL2 custom Mesa GPU acceleration..."
    ./scripts/platforms/run-wsl-gpu.sh
}

cmd_dev_wsl_gpu_fast() {
    log_section "Starting with WSL2 GPU acceleration (release mode)..."
    ./scripts/platforms/run-wsl-gpu.sh --release
}

cmd_test() {
    log_section "Running all tests..."
    cargo test
}

cmd_test_unit() {
    log_section "Running unit tests..."
    cargo test --lib
}

cmd_test_coverage() {
    log_section "Running tests with coverage..."
    if ! command -v cargo-tarpaulin &> /dev/null; then
        log_info "Installing cargo-tarpaulin..."
        cargo install cargo-tarpaulin
    fi
    cargo tarpaulin --out Html --output-dir ./coverage
    log_success "Coverage report generated at ./coverage/tarpaulin-report.html"
}

cmd_quality_check() {
    log_section "Running full quality pipeline..."
    cargo fmt --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo check
    cargo test
    log_success "Quality check passed!"
}

cmd_lint() {
    log_section "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
}

cmd_lint_fix() {
    log_section "Auto-fixing clippy issues..."
    cargo clippy --all-targets --all-features --fix --allow-dirty
}

cmd_format() {
    log_section "Checking formatting..."
    cargo fmt --check
}

cmd_format_fix() {
    log_section "Fixing formatting..."
    cargo fmt
}

cmd_type_check() {
    log_section "Running cargo check..."
    cargo check
}

cmd_validate() {
    log_section "Validating assets..."
    ./scripts/validate-assets.sh 2>/dev/null || log_warn "validate-assets.sh not found"
}

cmd_build() {
    log_section "Building debug version..."
    cargo build
}

cmd_build_all() {
    log_section "Building all targets..."
    cargo build --all-targets
}

cmd_build_release() {
    log_section "Building release version..."
    cargo build --release
}

cmd_ai_fix() {
    log_section "AI: Running auto-fix analysis..."
    cargo fix --allow-dirty
}

cmd_clean() {
    log_section "Cleaning build artifacts..."
    cargo clean
    rm -rf ./coverage
    rm -rf ./dist
    log_success "Clean complete!"
}

cmd_setup() {
    log_section "Setting up environment..."
    ./scripts/setup.sh 2>/dev/null || log_warn "setup.sh not found, using default setup"
    
    # Check Rust installation
    if ! command -v rustc &> /dev/null; then
        log_error "Rust not found! Please install Rust: https://rustup.rs/"
        exit 1
    fi
    
    log_info "Rust version: $(rustc --version)"
    log_info "Cargo version: $(cargo --version)"
    
    # Build to download dependencies
    cargo check
    
    log_success "Setup complete!"
}

cmd_stats() {
    log_section "Project Statistics:"
    echo ""
    echo -e "${CYAN}📁 Files by Type:${NC}"
    echo "  Rust source files: $(find src -name '*.rs' 2>/dev/null | wc -l)"
    echo "  Asset files: $(find assets -type f 2>/dev/null | wc -l)"
    echo ""
    echo -e "${CYAN}📏 Lines of Code:${NC}"
    find src -name '*.rs' -exec wc -l {} + 2>/dev/null | tail -1 || echo "  0"
    echo ""
    echo -e "${CYAN}📦 Dependencies:${NC}"
    cargo tree 2>/dev/null | wc -l | xargs echo "  Total dependency tree lines:"
}

cmd_health_check() {
    log_section "Health Check:"
    echo ""
    echo -e "${GREEN}✅ Rust toolchain:${NC}"
    rustc --version
    echo -e "${GREEN}✅ Cargo:${NC}"
    cargo --version
    echo ""
    echo -e "${GREEN}✅ Project structure:${NC}"
    test -f Cargo.toml && echo "  Cargo.toml: OK" || echo "  Cargo.toml: MISSING"
    test -d src && echo "  src/: OK" || echo "  src/: MISSING"
    test -d assets && echo "  assets/: OK" || echo "  assets/: MISSING"
    echo ""
    echo -e "${GREEN}✅ Dependencies:${NC}"
    cargo check 2>&1 | tail -1
}

cmd_check_deps() {
    log_section "Checking dependencies..."
    if command -v cargo-outdated &> /dev/null; then
        cargo outdated
    else
        log_warn "Install cargo-outdated for version checking: cargo install cargo-outdated"
        cargo tree | head -20
    fi
}

cmd_watch() {
    log_section "Starting watch mode..."
    if command -v cargo-watch &> /dev/null; then
        cargo watch -x run
    else
        log_warn "cargo-watch not found, running once..."
        log_info "Install with: cargo install cargo-watch"
        cargo run
    fi
}

cmd_dist() {
    log_section "Creating distribution..."
    mkdir -p dist
    cargo build --release
    if cp target/release/bevy-3d-renderer dist/ 2>/dev/null; then
        log_success "Distribution created at ./dist/bevy-3d-renderer"
    elif cp target/release/bevy_3d_renderer dist/ 2>/dev/null; then
        log_success "Distribution created at ./dist/bevy_3d_renderer"
    else
        log_warn "Binary name may differ - check target/release/"
        ls -la target/release/ | grep -E "^-.*x" | awk '{print $NF}' | head -5
    fi
}

# Main command dispatcher
case "${1:-help}" in
    dev)
        cmd_dev
        ;;
    dev:fast)
        cmd_dev_fast
        ;;
    dev:safe)
        cmd_dev_safe
        ;;
    dev:release)
        cmd_dev_release
        ;;
    dev:wsl:gpu)
        cmd_dev_wsl_gpu
        ;;
    dev:wsl:gpu:fast)
        cmd_dev_wsl_gpu_fast
        ;;
    test)
        cmd_test
        ;;
    test:unit)
        cmd_test_unit
        ;;
    test:coverage)
        cmd_test_coverage
        ;;
    quality-check)
        cmd_quality_check
        ;;
    lint)
        cmd_lint
        ;;
    lint:fix)
        cmd_lint_fix
        ;;
    format)
        cmd_format
        ;;
    format:fix)
        cmd_format_fix
        ;;
    type-check)
        cmd_type_check
        ;;
    validate|validate:data)
        cmd_validate
        ;;
    build)
        cmd_build
        ;;
    build:all)
        cmd_build_all
        ;;
    build:release)
        cmd_build_release
        ;;
    ai-fix|fix)
        cmd_ai_fix
        ;;
    clean)
        cmd_clean
        ;;
    setup)
        cmd_setup
        ;;
    stats)
        cmd_stats
        ;;
    health-check|health:check)
        cmd_health_check
        ;;
    check-deps)
        cmd_check_deps
        ;;
    check-gpu)
        ./scripts/check-gpu.sh
        ;;
    regression-test)
        ./scripts/regression-test.sh
        ;;
    watch)
        cmd_watch
        ;;
    dist)
        cmd_dist
        ;;
    help|--help|-h|"")
        show_help
        ;;
    *)
        log_error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac

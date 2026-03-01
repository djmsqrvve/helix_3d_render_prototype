# Helix 3D Renderer - Unified Command Interface
# Matches Helix2000 command conventions

.PHONY: help dev dev-fast dev-safe dev-animation dev-no-accessories test test-unit test-coverage quality-check lint lint-fix type-check validate build build-all build-release ai-test ai-docs ai-fix clean setup stats health-check check-deps screenshot screenshot-quick log-view test-all-animations test-accessories guardrail guardrail-strict

# Default target
help:
	@echo "🎮 Helix 3D Renderer - Command Interface"
	@echo ""
	@echo "🔥 Student Commands:"
	@echo "  make play         🚀 Just start the game and have fun!"
	@echo ""
	@echo "🚀 Quick Start Commands:"
	@echo "  make dev          Start with full checks"
	@echo "  make dev-fast     Fastest startup (skip checks)"
	@echo "  make dev-safe     Kill ports & start fresh"
	@echo "  make dev-release  Start optimized release build"
	@echo "  make dev-gpu      Start with custom Mesa GPU acceleration"
	@echo "  make dev-gpu-fast GPU accelerated + release mode"
	@echo "  make dev-animation N=2   Start with animation N (0-9)"
	@echo "  make dev-no-accessories  Start with all accessories hidden"
	@echo ""
	@echo "🧪 Testing Commands:"
	@echo "  make test         Run all tests"
	@echo "  make test-unit    Unit tests only"
	@echo "  make test-coverage Coverage report"
	@echo ""
	@echo "✅ Quality Commands:"
	@echo "  make quality-check Full quality pipeline"
	@echo "  make lint         Run clippy (Rust linter)"
	@echo "  make lint-fix     Auto-fix clippy issues"
	@echo "  make format       Check formatting"
	@echo "  make format-fix   Fix formatting"
	@echo "  make type-check   Type check (cargo check)"
	@echo "  make validate     Validate assets & data"
	@echo ""
	@echo "🛡️ Guardrail Commands:"
	@echo "  make guardrail       Quick safety checks"
	@echo "  make guardrail-strict Full safety checks"
	@echo ""
	@echo "🔨 Build Commands:"
	@echo "  make build        Debug build"
	@echo "  make build-all    Build all targets"
	@echo "  make build-release Release build (optimized)"
	@echo ""
	@echo "🤖 AI-Assisted Commands:"
	@echo "  make ai-test      Find test gaps"
	@echo "  make ai-docs      Update docs"
	@echo "  make ai-fix       Auto-fix analysis"
	@echo ""
	@echo "📸 Screenshot & Testing:"
	@echo "  make screenshot          Capture screenshot (frame 60)"
	@echo "  make screenshot-quick    Capture screenshot (frame 30)"
	@echo "  make test-all-animations Test all animations"
	@echo "  make test-accessories    Test each accessory"
	@echo "  make log-view            View recent log output"
	@echo ""
	@echo "🧹 Utility Commands:"
	@echo "  make clean        Clean build artifacts"
	@echo "  make setup        Setup environment"
	@echo "  make stats        Project statistics"
	@echo "  make health-check Health check"
	@echo "  make check-deps   Check dependencies"
	@echo "  make check-gpu    Check GPU acceleration status"
	@echo "  make regression-test Run regression tests"

# 🔥 Student Commands

play:
	@echo "🚀 Launching Helix 3D Renderer... HAVE FUN!"
	@cargo run --release --features egui

# 🚀 Quick Start Commands

dev:
	@echo "🔧 Starting development mode with checks..."
	@./scripts/check-deps.sh
	@cargo check
	@cargo run --features egui

dev-fast:
	@echo "⚡ Starting development mode (fast start)..."
	@cargo run --features egui

dev-cpu:
	@echo "🐢 Starting development mode (software rendering, with egui panel)..."
	@WGPU_BACKEND=gl cargo run --features egui

dev-safe:
	@echo "🧹 Safe dev mode - cleaning up first..."
	@make clean
	@./scripts/kill-ports.sh 2>/dev/null || true
	@cargo check
	@cargo run --features egui

dev-release:
	@echo "🚀 Starting optimized release build..."
	@cargo run --release --features egui

dev-wsl-gpu:
	@echo "🎮 Starting with WSL2 custom Mesa GPU acceleration..."
	@./scripts/platforms/run-wsl-gpu-custom.sh

dev-wsl-gpu-fast:
	@echo "🎮 Starting with WSL2 GPU acceleration (fast mode)..."
	@./scripts/platforms/run-wsl-gpu.sh --release

# 🪟 Windows Cross-Compilation & Testing

build-windows:
	@echo "🪟 Building for Windows (x64)..."
	@cargo build --release --target x86_64-pc-windows-gnu --features egui
	@echo "✅ Windows build complete: target/gltf_test/x86_64-pc-windows-gnu/release/bevy-3d-renderer.exe"

run-windows: build-windows
	@echo "🍷 Running Windows build via Wine..."
	@wine target/gltf_test/x86_64-pc-windows-gnu/release/bevy-3d-renderer.exe

dist-windows: build-windows
	@echo "📦 Packaging for Windows distribution..."
	@mkdir -p dist/windows
	@cp target/gltf_test/x86_64-pc-windows-gnu/release/bevy-3d-renderer.exe dist/windows/
	@cp -r assets dist/windows/
	@echo "✅ Distribution ready in ./dist/windows/"
	@echo "💡 To share: zip -r helix_windows.zip dist/windows/"

dev-animation:

	@echo "🎬 Starting with animation $(N)..."
	@cargo run -- --animation $(N)

dev-no-accessories:
	@echo "🏃 Starting with all accessories disabled..."
	@cargo run -- --disable-accessories "drow_armor,drow_bracer,drow_cape,drow_haircowl,drow_legs,drow_quiver,drow_weapon,drow_marksmanship_arrow"

# 🧪 Testing Commands

test:
	@echo "🧪 Running all tests..."
	@cargo test

test-unit:
	@echo "🧪 Running unit tests..."
	@cargo test --lib

test-coverage:
	@echo "📊 Running tests with coverage..."
	@which cargo-tarpaulin >/dev/null 2>&1 || (echo "Installing cargo-tarpaulin..."; cargo install cargo-tarpaulin)
	@cargo tarpaulin --out Html --output-dir ./coverage
	@echo "📊 Coverage report generated at ./coverage/tarpaulin-report.html"

# ✅ Quality Commands

quality-check:
	@echo "🔍 Running full quality pipeline..."
	@cargo fmt --check
	@cargo clippy --all-targets --all-features -- -D warnings
	@cargo check
	@cargo test
	@echo "✅ Quality check passed!"

lint:
	@echo "🔍 Running clippy (Rust linter)..."
	@cargo clippy --all-targets --all-features -- -D warnings

lint-fix:
	@echo "🔧 Auto-fixing clippy issues..."
	@cargo clippy --all-targets --all-features --fix --allow-dirty

format:
	@echo "📐 Checking formatting..."
	@cargo fmt --check

format-fix:
	@echo "📐 Fixing formatting..."
	@cargo fmt

type-check:
	@echo "🔍 Running cargo check..."
	@cargo check

type-check-strict:
	@echo "🔍 Running strict cargo check..."
	@cargo check --all-targets --all-features

validate:
	@echo "✅ Validating assets and data..."
	@./scripts/validate-assets.sh

# 🔨 Build Commands

build:
	@echo "🔨 Building debug version..."
	@cargo build

build-all:
	@echo "🔨 Building all targets..."
	@cargo build --all-targets

build-release:
	@echo "🔨 Building release version (optimized)..."
	@cargo build --release

# 🤖 AI-Assisted Commands

ai-fix:
	@echo "🤖 AI: Running auto-fix analysis..."
	@cargo fix --allow-dirty

# 🧹 Utility Commands

clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -rf ./coverage
	@rm -rf ./dist

setup:
	@echo "🔧 Setting up environment..."
	@./scripts/setup.sh

stats:
	@echo "📊 Project Statistics:"
	@echo ""
	@echo "📁 Files by Type:"
	@find src -name "*.rs" | wc -l | xargs echo "  Rust source files:"
	@find assets -type f 2>/dev/null | wc -l | xargs echo "  Asset files:"
	@echo ""
	@echo "📏 Lines of Code:"
	@find src -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1
	@echo ""
	@echo "📦 Dependencies:"
	@cargo tree | wc -l | xargs echo "  Total dependency tree lines:"

health-check:
	@echo "🏥 Health Check:"
	@echo ""
	@echo "✅ Rust toolchain:"
	@rustc --version
	@echo "✅ Cargo:"
	@cargo --version
	@echo ""
	@echo "✅ Project structure:"
	@test -f Cargo.toml && echo "  Cargo.toml: OK" || echo "  Cargo.toml: MISSING"
	@test -d src && echo "  src/: OK" || echo "  src/: MISSING"
	@test -d assets && echo "  assets/: OK" || echo "  assets/: MISSING"
	@echo ""
	@echo "✅ Dependencies:"
	@cargo check 2>&1 | tail -1

check-deps:
	@echo "📦 Checking dependencies..."
	@cargo outdated 2>/dev/null || echo "Install cargo-outdated for version checking: cargo install cargo-outdated"

check-gpu:
	@./scripts/check-gpu.sh

regression-test:
	@echo "🧪 Running regression tests..."
	@./scripts/regression-test.sh

# 🛡️ Guardrail Commands (Safety Checks)

guardrail:
	@echo "🛡️ Running guardrail checks..."
	@echo ""
	@echo "1️⃣  Checking default build (no egui)..."
	@cargo check --quiet || (echo "❌ FAILED: Default build broken!"; exit 1)
	@echo "✅ Default build OK"
	@echo ""
	@echo "2️⃣  Checking tests..."
	@cargo test --quiet || (echo "❌ FAILED: Tests broken!"; exit 1)
	@echo "✅ Tests OK"
	@echo ""
	@echo "3️⃣  Checking formatting..."
	@cargo fmt -- --check || (echo "⚠️  Formatting issues (run 'make format-fix')"; exit 0)
	@echo "✅ Formatting OK"
	@echo ""
	@echo "🛡️ All guardrails passed!"

guardrail-strict:
	@echo "🛡️ Running STRICT guardrail checks..."
	@echo ""
	@echo "1️⃣  Checking default build..."
	@cargo check --all-targets --all-features --quiet || (echo "❌ FAILED: Build broken!"; exit 1)
	@echo "✅ Build OK"
	@echo ""
	@echo "2️⃣  Running clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings || (echo "❌ FAILED: Clippy warnings!"; exit 1)
	@echo "✅ Clippy OK"
	@echo ""
	@echo "3️⃣  Running tests..."
	@cargo test --quiet || (echo "❌ FAILED: Tests broken!"; exit 1)
	@echo "✅ Tests OK"
	@echo ""
	@echo "4️⃣  Checking formatting..."
	@cargo fmt -- --check || (echo "❌ FAILED: Formatting issues!"; exit 1)
	@echo "✅ Formatting OK"
	@echo ""
	@echo "🛡️ All strict guardrails passed!"

# 📸 Screenshot & Testing Commands

screenshot:
	@echo "📸 Capturing screenshot..."
	@cargo run -- --screenshot screenshot_$(shell date +%Y%m%d_%H%M%S).png --capture-frame 60

screenshot-cloud:
	@echo "☁️  Capturing screenshot in the cloud (using Virtual Screen)..."
	@xvfb-run -a cargo run -- --screenshot screenshot_$(shell date +%Y%m%d_%H%M%S).png --capture-frame 60

test-all-animations:
	@echo "🎬 Testing all animations..."
	@mkdir -p test-outputs/animations
	@for i in 0 1 2 3 4 5 6 7 8 9; do \
		echo "  Capturing animation $$i..."; \
		cargo run --release -- --animation $$i --screenshot test-outputs/animations/anim_$$i.png --capture-frame 30; \
	done
	@echo "✅ Animation captures saved to test-outputs/animations/"

test-accessories:
	@echo "🎽 Testing individual accessories..."
	@mkdir -p test-outputs/accessories
	@echo "  Capturing base model (all accessories)..."
	@cargo run --release -- --screenshot test-outputs/accessories/base.png --capture-frame 30
	@echo "  Capturing with no accessories..."
	@cargo run --release -- --disable-accessories "drow_armor,drow_bracer,drow_cape,drow_haircowl,drow_legs,drow_quiver,drow_weapon,drow_marksmanship_arrow" --screenshot test-outputs/accessories/none.png --capture-frame 30
	@echo "✅ Accessory captures saved to test-outputs/accessories/"

screenshot-quick:
	@echo "📸 Quick screenshot (frame 30)..."
	@cargo run -- --screenshot screenshot_$(shell date +%Y%m%d_%H%M%S).png --capture-frame 30

log-view:
	@echo "📋 Recent log output:"
	@cargo run -- --verbose 2>&1 | tee run.log | tail -100

# Release/Deploy commands

dist:
	@echo "📦 Creating distribution..."
	@mkdir -p dist
	@cargo build --release
	@cp target/release/bevy-3d-renderer dist/ 2>/dev/null || cp target/release/bevy_3d_renderer dist/ 2>/dev/null || echo "Binary name may differ"
	@echo "✅ Distribution created in ./dist"

# Watch mode (requires cargo-watch)
watch:
	@echo "👀 Starting watch mode..."
	@which cargo-watch >/dev/null 2>&1 && cargo watch -x run || (echo "Install cargo-watch: cargo install cargo-watch"; cargo run)

watch-test:
	@echo "👀 Starting test watch mode..."
	@which cargo-watch >/dev/null 2>&1 && cargo watch -x test || (echo "Install cargo-watch: cargo install cargo-watch"; cargo test)

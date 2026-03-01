#!/bin/bash

# Helix 3D Renderer - Environment Setup Script

set -e

echo "🔧 Setting up Helix 3D Renderer environment..."

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Check Rust installation
echo "📦 Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}❌ Rust not found!${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}✅ Rust $(rustc --version)${NC}"
echo -e "${GREEN}✅ Cargo $(cargo --version)${NC}"

# Check for required dependencies
echo ""
echo "📦 Checking dependencies..."

# Check for graphics libraries (Linux)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Checking graphics libraries..."
    
    # Check for vulkan
    if ldconfig -p | grep -q libvulkan; then
        echo -e "${GREEN}✅ Vulkan found${NC}"
    else
        echo -e "${YELLOW}⚠️  Vulkan not found. Install with: sudo apt install libvulkan1${NC}"
    fi
    
    # Check for wayland/x11 deps
    if ldconfig -p | grep -q libwayland; then
        echo -e "${GREEN}✅ Wayland libraries found${NC}"
    else
        echo -e "${YELLOW}⚠️  Wayland libraries not found${NC}"
    fi
    
    if ldconfig -p | grep -q libxkbcommon; then
        echo -e "${GREEN}✅ xkbcommon found${NC}"
    else
        echo -e "${YELLOW}⚠️  xkbcommon not found. Install with: sudo apt install libxkbcommon-x11-dev${NC}"
    fi
    
    if ldconfig -p | grep -q libasound; then
        echo -e "${GREEN}✅ ALSA found${NC}"
    else
        echo -e "${YELLOW}⚠️  ALSA not found. Install with: sudo apt install libasound2-dev${NC}"
    fi
fi

# Check for optional tools
echo ""
echo "🔧 Checking optional tools..."

if command -v cargo-watch &> /dev/null; then
    echo -e "${GREEN}✅ cargo-watch installed${NC}"
else
    echo -e "${YELLOW}⚠️  cargo-watch not found. Install with: cargo install cargo-watch${NC}"
fi

if command -v cargo-tarpaulin &> /dev/null; then
    echo -e "${GREEN}✅ cargo-tarpaulin installed${NC}"
else
    echo -e "${YELLOW}⚠️  cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin${NC}"
fi

if command -v cargo-outdated &> /dev/null; then
    echo -e "${GREEN}✅ cargo-outdated installed${NC}"
else
    echo -e "${YELLOW}⚠️  cargo-outdated not found. Install with: cargo install cargo-outdated${NC}"
fi

# Verify project structure
echo ""
echo "📁 Checking project structure..."
if [ -f "Cargo.toml" ]; then
    echo -e "${GREEN}✅ Cargo.toml found${NC}"
else
    echo -e "${RED}❌ Cargo.toml not found${NC}"
    exit 1
fi

if [ -d "src" ]; then
    echo -e "${GREEN}✅ src/ directory found${NC}"
else
    echo -e "${RED}❌ src/ directory not found${NC}"
    exit 1
fi

if [ -d "assets" ]; then
    echo -e "${GREEN}✅ assets/ directory found${NC}"
else
    echo -e "${YELLOW}⚠️  assets/ directory not found${NC}"
fi

# Initial build to download dependencies
echo ""
echo "📦 Downloading dependencies..."
cargo check

echo ""
echo -e "${GREEN}✅ Setup complete!${NC}"
echo ""
echo "Next steps:"
echo "  ./run.sh dev        Start development server"
echo "  ./run.sh dev:fast   Fast start (skip checks)"
echo "  make dev            Alternative using make"

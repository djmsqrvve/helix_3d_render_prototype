#!/bin/bash

# Helix 3D Renderer - Regression Test Script
# Verifies the build and basic functionality

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASS=0
FAIL=0

test_pass() {
    echo -e "${GREEN}✅ PASS${NC}: $1"
    PASS=$((PASS + 1))
}

test_fail() {
    echo -e "${RED}❌ FAIL${NC}: $1"
    FAIL=$((FAIL + 1))
}

echo "🧪 Running Regression Tests..."
echo ""

# Test 1: Code compiles
echo "Test 1: Code compilation..."
if cargo check --quiet 2>/dev/null; then
    test_pass "Code compiles without errors"
else
    test_fail "Code compilation failed"
fi

# Test 2: Tests pass
echo "Test 2: Unit tests..."
if cargo test --quiet 2>/dev/null; then
    test_pass "All tests pass"
else
    test_fail "Some tests failed"
fi

# Test 3: Formatting is correct
echo "Test 3: Code formatting..."
if cargo fmt --check 2>/dev/null; then
    test_pass "Code is properly formatted"
else
    test_fail "Code formatting issues (run: cargo fmt)"
fi

# Test 4: No clippy warnings
echo "Test 4: Clippy linting..."
if cargo clippy --all-targets --all-features -- -D warnings 2>/dev/null; then
    test_pass "No clippy warnings"
else
    test_fail "Clippy warnings found"
fi

# Test 5: Assets exist
echo "Test 5: Required assets..."
if [ -f "assets/test_models/dota_models/models/heroes/drow/drow_base.gltf" ]; then
    test_pass "Drow base model exists"
else
    test_fail "Drow base model missing"
fi

# Test 6: Camera position in source
echo "Test 6: Camera position..."
if grep -q "from_xyz(0.0, 5.0, 10.0)" src/main.rs; then
    test_pass "Camera at correct position (0, 5, 10)"
else
    test_fail "Camera position changed"
fi

# Test 7: Window title
echo "Test 7: Window title..."
if grep -q "Drow Ranger - Complete Assembly" src/main.rs; then
    test_pass "Window title correct"
else
    test_fail "Window title changed"
fi

# Test 8: Animation count
echo "Test 8: Animation count..."
ANIM_COUNT=$(grep -c "(\".*\", [0-9]\+)" src/main.rs | head -1 || echo "0")
if [ "$ANIM_COUNT" -ge "10" ]; then
    test_pass "Has expected animations"
else
    test_fail "Animation count changed"
fi

echo ""
echo "===================="
echo "Results: $PASS passed, $FAIL failed"
echo "===================="

if [ $FAIL -gt 0 ]; then
    echo -e "${RED}❌ Regression tests FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}✅ All regression tests PASSED${NC}"
    exit 0
fi

#!/bin/bash

# Helix 3D Renderer - Asset Validation Script

set -e

echo "✅ Validating assets..."

ASSETS_DIR="./assets"

if [ ! -d "$ASSETS_DIR" ]; then
    echo "❌ assets/ directory not found"
    exit 1
fi

# Count assets
total_assets=$(find "$ASSETS_DIR" -type f | wc -l)
echo "Found $total_assets asset files"

# Check for required directories
echo ""
echo "Checking asset structure..."

if [ -d "$ASSETS_DIR/test_models" ]; then
    model_count=$(find "$ASSETS_DIR/test_models" -name "*.gltf" -o -name "*.glb" | wc -l)
    echo "  ✅ Models: $model_count GLTF/GLB files"
fi

# Validate GLTF files (basic check)
echo ""
echo "Validating GLTF files..."
for gltf in $(find "$ASSETS_DIR" -name "*.gltf"); do
    # Check if it's valid JSON (GLTF is JSON-based)
    if python3 -c "import json; json.load(open('$gltf'))" 2>/dev/null; then
        echo "  ✅ $(basename $gltf)"
    else
        echo "  ⚠️  $(basename $gltf) - may have issues"
    fi
done

echo ""
echo "✅ Asset validation complete"

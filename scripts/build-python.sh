#!/bin/bash
# Build Python package with maturin

set -e

echo "🐍 Building Python package..."

# Install maturin if not present
if ! command -v maturin &> /dev/null; then
    echo "Installing maturin..."
    pip install maturin
fi

# Build for current platform
echo "Building for current platform..."
maturin build --release

# Build wheels for multiple platforms (if on CI)
if [ "$CI" = "true" ]; then
    echo "Building universal wheels..."
    maturin build --release --universal2
fi

echo "✅ Python package built!"
echo "Wheels in: target/wheels/"

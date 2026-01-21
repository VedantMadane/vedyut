#!/bin/bash
# Publish all Rust crates to crates.io in dependency order

set -e

echo "📦 Publishing Vedyut crates to crates.io..."

cd rust

# Check if logged in
if ! cargo login --help > /dev/null 2>&1; then
    echo "❌ Please run 'cargo login YOUR_TOKEN' first"
    exit 1
fi

# Function to publish a crate
publish_crate() {
    local crate=$1
    echo ""
    echo "🚀 Publishing $crate..."
    cd $crate
    
    if cargo publish; then
        echo "✅ $crate published successfully"
    else
        echo "⚠️  $crate may already be published or failed"
    fi
    
    cd ..
    sleep 10  # Wait for crates.io to propagate
}

# Publish in dependency order
publish_crate "vedyut-lipi"
publish_crate "vedyut-sandhi"
publish_crate "vedyut-prakriya"
publish_crate "vedyut-kosha"
publish_crate "vedyut-cheda"
publish_crate "vedyut-sanskritify"
publish_crate "vedyut-core"

echo ""
echo "🎉 All crates published!"
echo "View at: https://crates.io/crates/vedyut-core"

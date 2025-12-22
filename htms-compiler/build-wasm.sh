#!/bin/bash
# Build WASM package with correct npm scope
# This ensures the package is always published as @progalaxyelabs/htms-compiler

set -e

echo "Building WASM with npm scope @progalaxyelabs..."
wasm-pack build --target nodejs --out-dir pkg --scope progalaxyelabs

echo ""
echo "✅ Build complete!"
echo "Generated package name:"
cat pkg/package.json | grep '"name"'
echo ""
echo "To publish: cd pkg && npm publish --access public"

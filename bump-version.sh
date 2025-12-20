#!/bin/bash

# HTMS Version Bump Script
# Usage: ./bump-version.sh <new-version>
# Example: ./bump-version.sh 0.3.0

if [ -z "$1" ]; then
  echo "❌ Error: Version number required"
  echo "Usage: ./bump-version.sh <version>"
  echo "Example: ./bump-version.sh 0.3.0"
  exit 1
fi

NEW_VERSION=$1

echo "🔄 Bumping version to $NEW_VERSION..."
echo ""

# Update Cargo.toml
echo "📝 Updating htms-compiler/Cargo.toml..."
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" htms-compiler/Cargo.toml

# Update CLI package.json
echo "📝 Updating htms-cli/package.json..."
sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" htms-cli/package.json
sed -i "s/\"@progalaxyelabs\/htms-compiler\": \"\^.*\"/\"@progalaxyelabs\/htms-compiler\": \"^$NEW_VERSION\"/" htms-cli/package.json

# Update VSCode extension package.json
echo "📝 Updating htms-vscode/package.json..."
sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" htms-vscode/package.json
sed -i "s/\"@progalaxyelabs\/htms-compiler\": \"\^.*\"/\"@progalaxyelabs\/htms-compiler\": \"^$NEW_VERSION\"/" htms-vscode/package.json

echo ""
echo "✅ Version bumped to $NEW_VERSION in all packages!"
echo ""

# Rebuild the compiler
echo "🔨 Rebuilding compiler..."
cd htms-compiler
npm run build > /dev/null 2>&1
cd ..

# Verify versions
echo ""
echo "🔍 Verifying versions:"
echo "  Compiler (pkg): $(grep '"version"' htms-compiler/pkg/package.json | head -1 | sed 's/.*: "\(.*\)".*/\1/')"
echo "  CLI:            $(grep '"version"' htms-cli/package.json | head -1 | sed 's/.*: "\(.*\)".*/\1/')"
echo "  VSCode:         $(grep '"version"' htms-vscode/package.json | head -1 | sed 's/.*: "\(.*\)".*/\1/')"

echo ""
echo "✅ All set! Version $NEW_VERSION is ready to publish."
echo ""
echo "📦 Next steps:"
echo "  1. Commit: git add -A && git commit -m 'Bump version to $NEW_VERSION'"
echo "  2. Tag: git tag -a v$NEW_VERSION -m 'Release v$NEW_VERSION'"
echo "  3. Publish compiler: cd htms-compiler/pkg && npm publish --access public"
echo "  4. Publish CLI: cd htms-cli && npm publish --access public"
echo "  5. Push: git push origin main && git push origin v$NEW_VERSION"

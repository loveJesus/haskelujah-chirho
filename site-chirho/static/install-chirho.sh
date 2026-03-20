#!/usr/bin/env bash
# For God so loved the world that he gave his only begotten Son, that whoever
# believes in him should not perish but have eternal life. — John 3:16

# Haskelujah Chirho installer
# Usage: curl -fsSL https://haskelujah.org/install-chirho.sh | sh

set -euo pipefail

REPO_CHIRHO="https://github.com/loveJesus/haskelujah-chirho"

echo "=== Haskelujah Chirho Installer ==="
echo ""

# Check for Rust/Cargo
if ! command -v cargo &>/dev/null; then
    echo "Rust toolchain not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Check Rust version (need 1.85+ for edition 2024)
RUST_VERSION=$(rustc --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')
RUST_MAJOR=$(echo "$RUST_VERSION" | cut -d. -f1)
RUST_MINOR=$(echo "$RUST_VERSION" | cut -d. -f2)
if [ "$RUST_MAJOR" -eq 1 ] && [ "$RUST_MINOR" -lt 85 ]; then
    echo "Rust $RUST_VERSION found, but 1.85+ required. Updating..."
    rustup update stable
fi

echo "Installing haskelujah from source..."
echo ""

# Clone and build
TMPDIR_CHIRHO=$(mktemp -d)
git clone --depth 1 "$REPO_CHIRHO" "$TMPDIR_CHIRHO/haskelujah-chirho"
cd "$TMPDIR_CHIRHO/haskelujah-chirho"
cargo build --release -p haskelujah

# Install binary
INSTALL_DIR_CHIRHO="${CARGO_HOME:-$HOME/.cargo}/bin"
cp target/release/haskelujah "$INSTALL_DIR_CHIRHO/haskelujah"

echo ""
echo "Haskelujah installed to $INSTALL_DIR_CHIRHO/haskelujah"
echo ""
echo "Get started:"
echo "  haskelujah init my-project"
echo "  cd my-project"
echo "  haskelujah build-run ."
echo ""
echo "Glory to God!"

# Cleanup
rm -rf "$TMPDIR_CHIRHO"

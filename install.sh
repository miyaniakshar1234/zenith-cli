#!/bin/bash

set -e

REPO="miyaniakshar1234/zenith-cli"
BINARY="zenith-cli"
INSTALL_DIR="$HOME/.local/bin"

# Detect OS and Arch
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        OS_TYPE="linux"
        ;;
    Darwin)
        OS_TYPE="macos"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64)
        ARCH_TYPE="amd64"
        ;;
    arm64|aarch64)
        # Assuming we build aarch64, if not fallback to amd64 (Rosetta on Mac)
        # Current release workflow only builds amd64 for Mac/Linux explicitly in matrix
        # So we force amd64 for now unless we update workflow
        ARCH_TYPE="amd64" 
        ;;
    *)
        echo "Unsupported Architecture: $ARCH"
        exit 1
        ;;
esac

ASSET_NAME="zenith-cli-${OS_TYPE}-${ARCH_TYPE}"
if [ "$OS_TYPE" = "windows" ]; then
    ASSET_NAME="${ASSET_NAME}.exe"
fi

echo "🚀 Detecting system..."
echo "   OS: $OS_TYPE"
echo "   Arch: $ARCH_TYPE"

# Get Latest Release URL (using GitHub API would be better but rate limits; direct latest download link is easier)
# Actually, GitHub "latest" release redirect structure:
# https://github.com/user/repo/releases/latest/download/asset_name

DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET_NAME}"

echo "⬇️  Downloading Zenith CLI..."
mkdir -p "$INSTALL_DIR"
curl -sL "$DOWNLOAD_URL" -o "$INSTALL_DIR/$BINARY"

chmod +x "$INSTALL_DIR/$BINARY"

echo "✅ Installed to $INSTALL_DIR/$BINARY"

# Check PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "⚠️  Warning: $INSTALL_DIR is not in your PATH."
    echo "   Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
    echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
else
    echo "🎉 Ready! Run 'zenith-cli' to start."
fi

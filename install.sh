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
        ARCH_TYPE="amd64" # Fallback/Rosetta for now as we only release amd64
        ;;
    *)
        echo "Unsupported Architecture: $ARCH"
        exit 1
        ;;
esac

ASSET_NAME="zenith-cli-${OS_TYPE}-${ARCH_TYPE}"

echo "🚀 Detecting system..."
echo "   OS: $OS_TYPE"
echo "   Arch: $ARCH_TYPE"

DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET_NAME}"

echo "⬇️  Downloading Zenith CLI..."
mkdir -p "$INSTALL_DIR"
curl -sL "$DOWNLOAD_URL" -o "$INSTALL_DIR/$BINARY"

chmod +x "$INSTALL_DIR/$BINARY"

echo "✅ Installed to $INSTALL_DIR/$BINARY"

# Smart PATH Setup
SHELL_CONFIG=""
if [ -f "$HOME/.zshrc" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -f "$HOME/.bashrc" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
elif [ -f "$HOME/.bash_profile" ]; then
    SHELL_CONFIG="$HOME/.bash_profile"
fi

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "🔧 Adding to PATH..."
    if [ -n "$SHELL_CONFIG" ]; then
        echo "" >> "$SHELL_CONFIG"
        echo "# Zenith CLI" >> "$SHELL_CONFIG"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
        echo "✅ Added to $SHELL_CONFIG"
        echo "👉 Run 'source $SHELL_CONFIG' or restart terminal to use it."
    else
        echo "⚠️  Could not detect shell config. Run this manually:"
        echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
else
    echo "🎉 Path is already correct."
fi

echo ""
echo "🚀 Installation Complete! Run 'zenith-cli' to start."

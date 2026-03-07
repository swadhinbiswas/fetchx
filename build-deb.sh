#!/bin/bash
# Build DEB package for fetchx

set -e

VERSION="0.2.0"
ARCH="amd64"
PACKAGE_NAME="fetchx_${VERSION}_${ARCH}"

echo "Building fetchx DEB package..."

# Build the binary
cargo build --release --locked

# Create package directory structure
mkdir -p "$PACKAGE_NAME/usr/bin"
mkdir -p "$PACKAGE_NAME/usr/share/applications"
mkdir -p "$PACKAGE_NAME/usr/share/bash-completion/completions"
mkdir -p "$PACKAGE_NAME/usr/share/zsh/vendor-completions"
mkdir -p "$PACKAGE_NAME/usr/share/man/man1"

# Install binary
install -Dm755 target/release/fetchx "$PACKAGE_NAME/usr/bin/fetchx"

# Install desktop file
install -Dm644 debian/fetchx.desktop "$PACKAGE_NAME/usr/share/applications/fetchx.desktop"

# Install completions
install -Dm644 debian/fetchx.bash-completion "$PACKAGE_NAME/usr/share/bash-completion/completions/fetchx"
install -Dm644 debian/fetchx.zsh-completion "$PACKAGE_NAME/usr/share/zsh/vendor-completions/_fetchx"

# Install man page
install -Dm644 debian/fetchx.1 "$PACKAGE_NAME/usr/share/man/man1/fetchx.1"

# Create DEB package
dpkg-deb --build "$PACKAGE_NAME"

echo "Package created: $PACKAGE_NAME.deb"
echo ""
echo "To install:"
echo "  sudo dpkg -i $PACKAGE_NAME.deb"
echo "  sudo apt-get install -f  # to install dependencies"

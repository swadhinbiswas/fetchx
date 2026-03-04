#!/usr/bin/env bash
# ============================================================================
# FetchX Installer Script
# A fast, feature-rich system information tool written in Rust
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/swadhinbiswas/fetchx/main/install.sh | bash
#   or
#   wget -qO- https://raw.githubusercontent.com/swadhinbiswas/fetchx/main/install.sh | bash
#   or
#   ./install.sh
#
# Options:
#   --prefix=/usr/local    Installation prefix (default: ~/.local)
#   --system               Install system-wide to /usr/local/bin (requires sudo)
#   --uninstall            Remove FetchX
#   --no-alias             Skip shell alias setup
#   --help                 Show help
# ============================================================================

set -euo pipefail

# ── Colors ────────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

# ── Defaults ──────────────────────────────────────────────────────────────────
PREFIX="$HOME/.local"
SYSTEM_INSTALL=false
UNINSTALL=false
SETUP_ALIAS=true
REPO_URL="https://github.com/swadhinbiswas/fetchx.git"
BINARY_NAME="fetchx"

# ── Parse arguments ──────────────────────────────────────────────────────────
for arg in "$@"; do
    case "$arg" in
        --prefix=*)   PREFIX="${arg#*=}" ;;
        --system)     SYSTEM_INSTALL=true; PREFIX="/usr/local" ;;
        --uninstall)  UNINSTALL=true ;;
        --no-alias)   SETUP_ALIAS=false ;;
        --help|-h)
            echo "FetchX Installer"
            echo ""
            echo "Usage: ./install.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --prefix=PATH   Installation prefix (default: ~/.local)"
            echo "  --system        Install system-wide to /usr/local (requires sudo)"
            echo "  --uninstall     Remove FetchX"
            echo "  --no-alias      Skip shell alias setup"
            echo "  --help          Show this help"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $arg${RESET}"
            exit 1
            ;;
    esac
done

BINDIR="$PREFIX/bin"

# ── Helper functions ─────────────────────────────────────────────────────────
info()    { echo -e "${BLUE}::${RESET} $*"; }
success() { echo -e "${GREEN}✓${RESET} $*"; }
warn()    { echo -e "${YELLOW}!${RESET} $*"; }
error()   { echo -e "${RED}✗${RESET} $*"; exit 1; }

command_exists() { command -v "$1" &>/dev/null; }

maybe_sudo() {
    if [[ "$SYSTEM_INSTALL" == true ]]; then
        sudo "$@"
    else
        "$@"
    fi
}

# ── Uninstall ────────────────────────────────────────────────────────────────
if [[ "$UNINSTALL" == true ]]; then
    info "Uninstalling FetchX..."

    if [[ -f "$BINDIR/$BINARY_NAME" ]]; then
        maybe_sudo rm -f "$BINDIR/$BINARY_NAME"
        success "Removed $BINDIR/$BINARY_NAME"
    else
        warn "Binary not found at $BINDIR/$BINARY_NAME"
    fi

    # Remove config (ask first)
    if [[ -d "$HOME/.config/fetchx" ]]; then
        read -rp "Remove config directory (~/.config/fetchx)? [y/N] " answer
        if [[ "$answer" =~ ^[Yy]$ ]]; then
            rm -rf "$HOME/.config/fetchx"
            success "Removed ~/.config/fetchx"
        fi
    fi

    # Remove cache
    if [[ -d "$HOME/.cache/fetchx" ]]; then
        rm -rf "$HOME/.cache/fetchx"
        success "Removed ~/.cache/fetchx"
    fi

    success "FetchX uninstalled!"
    exit 0
fi

# ── Banner ───────────────────────────────────────────────────────────────────
echo -e "${CYAN}${BOLD}"
echo "  ╔═══════════════════════════════════════╗"
echo "  ║        ⚡ FetchX Installer ⚡         ║"
echo "  ║  Fast system info tool written in Rust ║"
echo "  ╚═══════════════════════════════════════╝"
echo -e "${RESET}"

# ── Check prerequisites ─────────────────────────────────────────────────────
info "Checking prerequisites..."

if ! command_exists rustc || ! command_exists cargo; then
    warn "Rust toolchain not found."
    echo ""
    echo -e "  Install Rust with: ${BOLD}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${RESET}"
    echo ""
    read -rp "Install Rust now? [Y/n] " answer
    if [[ ! "$answer" =~ ^[Nn]$ ]]; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        success "Rust installed!"
    else
        error "Rust is required to build FetchX. Aborting."
    fi
fi

RUST_VERSION=$(rustc --version | awk '{print $2}')
success "Rust $RUST_VERSION found"

if ! command_exists git; then
    error "git is required. Install it with your package manager."
fi
success "git found"

# ── Clone or update repository ───────────────────────────────────────────────
TMPDIR=$(mktemp -d)
CLONE_DIR="$TMPDIR/fetchx"

info "Cloning FetchX repository..."
if git clone --depth 1 "$REPO_URL" "$CLONE_DIR" 2>/dev/null; then
    success "Repository cloned"
    cd "$CLONE_DIR/fetchx"  # the Rust project is in the fetchx/ subdirectory
else
    # If clone fails, try building from current directory (local install)
    if [[ -f "Cargo.toml" ]] && grep -q "fetchx" Cargo.toml 2>/dev/null; then
        info "Building from local source..."
        CLONE_DIR="."
    else
        error "Failed to clone repository and no local source found."
    fi
fi

# ── Build ────────────────────────────────────────────────────────────────────
info "Building FetchX (release mode with LTO)..."
echo -e "  ${YELLOW}This may take 1-2 minutes on first build...${RESET}"

cargo build --release 2>&1 | while IFS= read -r line; do
    if [[ "$line" == *"Compiling"* ]]; then
        echo -e "  ${CYAN}$line${RESET}"
    fi
done

if [[ ! -f "target/release/$BINARY_NAME" ]]; then
    error "Build failed — binary not found."
fi

BINARY_SIZE=$(du -h "target/release/$BINARY_NAME" | awk '{print $1}')
success "Build complete! Binary size: $BINARY_SIZE"

# ── Install binary ───────────────────────────────────────────────────────────
info "Installing to $BINDIR..."
maybe_sudo mkdir -p "$BINDIR"
maybe_sudo install -m 755 "target/release/$BINARY_NAME" "$BINDIR/$BINARY_NAME"
success "Installed $BINDIR/$BINARY_NAME"

# ── Create default config ───────────────────────────────────────────────────
CONFIG_DIR="$HOME/.config/fetchx"
CONFIG_FILE="$CONFIG_DIR/config.toml"

if [[ ! -f "$CONFIG_FILE" ]]; then
    info "Creating default configuration..."
    mkdir -p "$CONFIG_DIR"
    "$BINDIR/$BINARY_NAME" --create-config 2>/dev/null || true
    success "Config created at $CONFIG_FILE"
else
    success "Config already exists at $CONFIG_FILE"

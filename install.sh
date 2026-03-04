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

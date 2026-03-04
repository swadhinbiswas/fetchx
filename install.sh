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

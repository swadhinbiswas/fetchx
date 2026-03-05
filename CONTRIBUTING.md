# Contributing to FetchX

First off, thank you for considering contributing to FetchX! Every contribution matters — whether it's a bug report, a new distro logo, a feature implementation, or documentation improvement.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Adding Distro Logos](#adding-distro-logos)
  - [Code Contributions](#code-contributions)
- [Code Style & Guidelines](#code-style--guidelines)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Development Tips](#development-tips)

---

## Getting Started

### Prerequisites

- **Rust** (2021 edition) — install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Git**
- A Linux, macOS, or BSD system for testing

### Development Setup

```bash
# 1. Fork the repo on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/fetchx.git
cd fetchx/fetchx

# 2. Build in debug mode (faster compilation)
cargo build

# 3. Run tests
cargo test

# 4. Run fetchx locally
cargo run

# 5. Build optimized release
cargo build --release
```

---

## Project Structure

```
fetchx/
├── Cargo.toml              # Dependencies and project metadata
├── Makefile                 # Build/install shortcuts
├── install.sh              # User install script
├── assets/
│   └── config.example.toml # Example configuration file
├── src/
│   ├── main.rs             # Entry point, CLI args, main flow
│   ├── ascii/
│   │   └── mod.rs          # 60+ distro ASCII art logos
│   ├── colors/
│   │   └── mod.rs          # Color schemes, gradients, palette
│   ├── config/
│   │   └── mod.rs          # TOML config parsing, defaults
│   ├── display/
│   │   └── mod.rs          # Render engine: side-by-side layout
│   ├── image_backend/
│   │   └── mod.rs          # Kitty, sixel, chafa, w3m, iTerm2
│   ├── image_cache.rs      # API image download & smart caching
│   ├── system/
│   │   └── mod.rs          # System info detection (OS, CPU, etc.)
│   └── utils/
│       └── mod.rs          # Unicode width, padding, truncation
└── .github/
    └── workflows/
        ├── ci.yml          # CI: lint, test, coverage, build
        └── release.yml     # Auto-release on tag push
```

---

## How to Contribute

### Reporting Bugs

Found a bug? Please [open an issue](https://github.com/swadhinbiswas/fetchx/issues/new?template=bug_report.yml) with:

1. Your FetchX version (`fetchx --version`)
2. Your OS, terminal, and DE/WM
3. Your config file (relevant parts)
4. Steps to reproduce
5. Expected vs actual behavior
6. Terminal output or screenshots

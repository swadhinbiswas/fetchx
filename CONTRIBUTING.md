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

### Suggesting Features

Have an idea? [Open a feature request](https://github.com/swadhinbiswas/fetchx/issues/new?template=feature_request.yml). Great features to work on:

- New system info detections
- Image backend improvements
- Config options
- Platform support (macOS, BSD, WSL)
- Status bar integrations (Waybar, Polybar, EWW)

### Adding Distro Logos

This is the easiest way to contribute! ASCII logos are in `src/ascii/mod.rs`.

**Steps:**

1. Find the distro's logo (check [neofetch's source](https://github.com/dylanaraps/neofetch/blob/master/neofetch) for reference)

2. Convert to ASCII art (roughly 18-25 lines tall, ~40 chars wide)

3. Use `{c1}` through `{c6}` for color placeholders

4. Add to `src/ascii/mod.rs`:

```rust
// In the get_ascii_art() match statement:
"mydistro" | "my_distro_id" => (vec![4, 6], r#"
{c1}         /\
{c1}        /  \
{c1}       /    \
{c1}      /      \
{c2}     /________\
{c2}    /          \
{c2}   /____________\
"#.to_string()),
```

5. The first element `vec![4, 6]` is the color list (terminal color numbers)

6. The distro ID must match the `ID` field in `/etc/os-release`

7. Test with: `cargo run -- --ascii-distro mydistro`

### Code Contributions

1. Check [open issues](https://github.com/swadhinbiswas/fetchx/issues) for tasks labeled `good first issue` or `help wanted`
2. Comment on the issue to claim it
3. Fork, branch, code, test, PR

---

## Code Style & Guidelines

### Rust Style

- **Format** with `cargo fmt` before committing
- **Lint** with `cargo clippy` — zero warnings required
- **No `unwrap()`** in production code — use `?`, `.unwrap_or()`, or `.ok()`
- **Error handling**: use descriptive error messages with context

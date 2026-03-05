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

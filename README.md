<!--
  FetchX — Fast Neofetch Alternative Written in Rust
  Keywords: neofetch, neofetch alternative, system info, fetch, rust, terminal,
            linux, ascii art, kitty, ricing, unixporn, dotfiles, system fetch,
            fastfetch alternative, screenfetch, pfetch, macchina
-->

<h1 align="center">
  <br>
  ⚡ FetchX
  <br>
</h1>

<h4 align="center">A blazing-fast, feature-rich system information tool written in Rust.<br>The modern neofetch replacement you've been waiting for.</h4>

<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"></a>
  <a href="https://github.com/swadhinbiswas/fetchx/releases"><img src="https://img.shields.io/badge/version-0.2.0-green.svg" alt="Version 0.2.0"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust 2021"></a>
  <a href="https://github.com/swadhinbiswas/fetchx/actions"><img src="https://img.shields.io/badge/tests-45%20passing-brightgreen.svg" alt="45 Tests Passing"></a>
  <a href="https://github.com/swadhinbiswas/fetchx/stargazers"><img src="https://img.shields.io/github/stars/swadhinbiswas/fetchx?style=social" alt="GitHub Stars"></a>
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#features">Features</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#gallery">Gallery</a> •
  <a href="#shell-aliases-replace-neofetch">Replace Neofetch</a>
</p>

<!-- Hero Screenshot -->
<p align="center">
  <img src="assets/image.png" alt="FetchX — neofetch alternative in Rust showing system info with ASCII art" width="700">
</p>

---

## Gallery

<table>
  <tr>
    <td align="center" width="50%">
      <img src="assets/image.png" alt="FetchX ASCII art mode with CachyOS" width="100%">
      <br><b>ASCII Art Mode</b>
      <br><sub>Classic distro logo with system info</sub>
    </td>
    <td align="center" width="50%">
      <img src="assets/image2.png" alt="FetchX with kitty image backend showing anime image" width="100%">
      <br><b>Kitty Image Backend</b>
      <br><sub>Auto-fetched image via smart caching</sub>
    </td>
  </tr>
  <tr>
    <td align="center" width="50%">
      <img src="assets/randomimage.png" alt="FetchX random image fetching — new image every run" width="100%">
      <br><b>Random Image Every Run</b>
      <br><sub>Background download, zero delay</sub>
    </td>
    <td align="center" width="50%">
      <img src="assets/random2.png" alt="FetchX with different random image from API" width="100%">
      <br><b>Smart Image Caching</b>
      <br><sub>Previous image shown instantly</sub>
    </td>
  </tr>
  <tr>
    <td align="center" width="50%">
      <img src="assets/setimage.png" alt="FetchX with custom image set by user" width="100%">
      <br><b>Custom Image</b>
      <br><sub>Use your own wallpaper or photo</sub>
    </td>
    <td align="center" width="50%">
      <img src="assets/image-copy.png" alt="FetchX Nerd Font icons and progress bars" width="100%">
      <br><b>Nerd Fonts + Progress Bars</b>
      <br><sub>Memory & disk usage bars with icons</sub>
    </td>
  </tr>
</table>

---

## Table of Contents

- [Gallery](#gallery)
- [Features](#features)
- [Installation](#installation)
  - [Quick Install (Recommended)](#quick-install-recommended)
  - [From Source (Manual)](#from-source-manual)
  - [With Make](#with-make)
  - [From Cargo](#from-cargo)
  - [Arch Linux / CachyOS / Manjaro (AUR)](#arch-linux--cachyos--manjaro-aur)
- [Shell Aliases (Replace neofetch)](#shell-aliases-replace-neofetch)
- [Configuration](#configuration)
  - [Getting Started](#getting-started)
  - [Config File Location](#config-file-location)
  - [Full Configuration Reference](#full-configuration-reference)
  - [Example Configs](#example-configs)
- [Usage](#usage)
- [Image Backends](#image-backends)
  - [How Smart Image Fetching Works](#how-smart-image-fetching-works)
- [Advanced Features](#advanced-features)
- [Information Displayed](#information-displayed)
- [Supported Distributions](#supported-distributions)
- [Comparison with Neofetch](#comparison-with-neofetch)
- [Dependencies](#dependencies)
- [Uninstalling](#uninstalling)
- [Contributing](#contributing)
- [Star History](#star-history)
- [License](#license)

---

## Features

> **Why FetchX over neofetch?** Neofetch is [no longer maintained](https://github.com/dylanaraps/neofetch/issues/2691). FetchX is a modern, actively developed replacement written in Rust — 10x faster, with image support and smart caching built in.

- **Blazing Fast** — Parallel info detection across 8 threads, finishes in ~10-50ms (vs neofetch's ~200-500ms)
- **Tiny Binary** — ~1.8 MB stripped, LTO-optimized — no Python, no Bash, no runtime deps
- **TOML Config** — Clean `~/.config/fetchx/config.toml` with 35+ options (no scripting needed)
- **5 Image Backends** — Kitty graphics protocol, Sixel, Chafa, w3m, iTerm2
- **Smart Image Caching** — Auto-downloads random images from API, shows instantly with zero delay, new image every run
- **60+ Distro Logos** — ASCII art with `{c1}`-`{c6}` color placeholders
- **Nerd Font Icons** — Optional icons for every info label (requires [Nerd Font](https://www.nerdfonts.com/))
- **Progress Bars** — Visual bars for memory and disk usage `[███████░░░░]`
- **Emoji Mode** — Fun emoji art alternative to ASCII logos
- **JSON Output** — Machine-readable `--json` flag for scripting & automation
- **Custom ASCII Art** — Load your own art from any text file
- **256-Color Support** — Full terminal color palette
- **Terminal-Width Aware** — Lines truncate cleanly, never wraps
- **Interactive Image Picker** — Browse & select images with fzf
- **Daemon Mode** — Background service for Waybar/Polybar/status bar integration
- **Drop-in neofetch replacement** — `alias neofetch='fetchx'` and you're done

---

## Installation

### Quick Install (Recommended)

One-liner that clones, builds, installs, and sets up aliases:

```bash
curl -fsSL https://raw.githubusercontent.com/swadhinbiswas/fetchx/main/install.sh | bash
```

Or with wget:

```bash
wget -qO- https://raw.githubusercontent.com/swadhinbiswas/fetchx/main/install.sh | bash
```

**Install script options:**

```bash
./install.sh                  # Install to ~/.local/bin (default)
./install.sh --system         # Install to /usr/local/bin (requires sudo)
./install.sh --prefix=/opt    # Custom prefix
./install.sh --no-alias       # Skip shell alias setup
./install.sh --uninstall      # Remove FetchX completely
```

### From Source (Manual)

**Prerequisites:** [Rust toolchain](https://rustup.rs/) (rustc + cargo)

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/swadhinbiswas/fetchx.git
cd fetchx/fetchx

# Build optimized release binary
cargo build --release

# Install to user directory (no sudo needed)
mkdir -p ~/.local/bin
install -m 755 target/release/fetchx ~/.local/bin/fetchx

# Make sure ~/.local/bin is in your PATH (see Shell Setup below)
```

**System-wide install (requires sudo):**

```bash
sudo install -m 755 target/release/fetchx /usr/local/bin/fetchx
```

### With Make

```bash
git clone https://github.com/swadhinbiswas/fetchx.git
cd fetchx/fetchx
make                    # Build
sudo make install       # Install to /usr/local/bin
# or
make PREFIX=~/.local install   # Install to ~/.local/bin
```

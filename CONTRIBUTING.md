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
- Document public functions with `///` doc comments
- Keep functions focused and small (< 50 lines ideally)

### Commit Messages

Use clear, descriptive commit messages:

```
feat: add CachyOS ASCII art logo
fix: resolve kitty image not displaying on first run
docs: update config reference with new options
refactor: extract GPU detection into separate function
test: add unit tests for uptime parsing
ci: add codecov integration
```

Prefixes: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `ci:`, `chore:`

### Branch Naming

```
feat/add-bazzite-logo
fix/kitty-image-crash
docs/update-readme
```

---

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_parse_uptime

# Run tests with output
cargo test -- --nocapture

# Run tests in a specific module
cargo test system::tests
```

### Writing Tests

Add tests in the same file using `#[cfg(test)]` modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function("input");
        assert_eq!(result, "expected output");
    }
}
```

### What to Test

- Parsing functions (uptime, memory, CPU info)
- Config loading and defaults
- ASCII art color placeholder substitution
- Utility functions (padding, truncation, width)

### Coverage

CI runs `cargo-tarpaulin` and uploads to [Codecov](https://codecov.io). Check coverage before submitting:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --all-features
```

---

## Pull Request Process

1. **Fork** the repository and create a feature branch from `main`

2. **Make your changes** — keep PRs focused on one thing

3. **Run checks locally:**

   ```bash
   cargo fmt --all -- --check   # Format check
   cargo clippy -- -D warnings  # Lint check
   cargo test                   # All tests pass
   cargo build --release        # Clean release build (0 warnings)
   ```

4. **Push** to your fork and open a PR against `main`

5. **Fill out the PR template** — describe changes, link issues, add screenshots

6. **Wait for CI** — all checks must pass (lint, test, build)

7. **Address review feedback** — maintainers may request changes

8. **Merge!** — once approved, a maintainer will merge your PR

### PR Tips

- Keep diffs small and focused — large PRs are harder to review
- Add screenshots for visual changes (new logos, layout changes)
- If adding a feature, update the README and config example
- If fixing a bug, add a test that catches the regression

---

## Development Tips

### Quick Iteration

```bash
# Watch mode — rebuilds on file changes (install cargo-watch first)
cargo install cargo-watch
cargo watch -x run

# Test a specific distro logo
cargo run -- --ascii-distro arch

# Test with kitty image backend
cargo run -- --backend kitty

# Test JSON output
cargo run -- --json | jq .

# Check binary size
cargo build --release && ls -lh target/release/fetchx
```

### Debugging

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo run

# Check what info is detected
cargo run -- --json 2>/dev/null | jq .

# Test config loading
cargo run -- --print-config
```

### Adding a New Info Field

1. Add detection logic in `src/system/mod.rs`
2. Add `show_my_field: bool` to `Config` struct in `src/config/mod.rs`
3. Add display logic in `src/display/mod.rs`
4. Add Nerd Font icon in the icon mapping
5. Add toggle to example config in `assets/config.example.toml`
6. Add tests for the detection
7. Update README documentation

---

## Community

- **Issues**: [GitHub Issues](https://github.com/swadhinbiswas/fetchx/issues)
- **Discussions**: [GitHub Discussions](https://github.com/swadhinbiswas/fetchx/discussions)
- **Share your rice**: Post screenshots on [r/unixporn](https://www.reddit.com/r/unixporn/) with `fetchx` tag!

---

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

**Thank you for making FetchX better!** Every contribution, no matter how small, helps the project grow. ⚡

# FetchX Installation Guide

## Quick Install

### From Source (Recommended for Development)

```bash
# Clone or navigate to the fetchx directory
cd fetchx

# Build in release mode
cargo build --release

# Copy to system path (requires sudo)
sudo cp target/release/fetchx /usr/local/bin/

# Verify installation
fetchx --version
```

### Using Cargo (When Published)

```bash
cargo install fetchx
```

### Arch Linux (AUR) - Coming Soon

```bash
# Using yay
yay -S fetchx

# Using paru
paru -S fetchx

# Using makepkg
git clone https://aur.archlinux.org/fetchx.git
cd fetchx
makepkg -si
```

### Homebrew (macOS) - Coming Soon

```bash
brew install fetchx
```

### NixOS - Coming Soon

```nix
# In your configuration.nix or flake
environment.systemPackages = [
  pkgs.fetchx
];
```

### Debian/Ubuntu (.deb package) - Coming Soon

```bash
# Download the .deb file
wget https://github.com/swadhinbiswas/fetchx/releases/download/v0.1.0/fetchx_0.1.0_amd64.deb
sudo dpkg -i fetchx_0.1.0_amd64.deb
```

### Fedora/RPM - Coming Soon

```bash
# Using dnf
sudo dnf install fetchx

# Or install RPM directly
sudo dnf install https://github.com/swadhinbiswas/fetchx/releases/download/v0.1.0/fetchx-0.1.0.x86_64.rpm
```

## Manual Installation

1. **Download the binary**

   ```bash
   wget https://github.com/swadhinbiswas/fetchx/releases/download/v0.1.0/fetchx-linux-x86_64.tar.gz
   tar xzf fetchx-linux-x86_64.tar.gz
   ```

2. **Move to PATH**

   ```bash
   sudo mv fetchx /usr/local/bin/
   sudo chmod +x /usr/local/bin/fetchx
   ```

3. **Verify**
   ```bash
   fetchx --version
   ```

## Configuration

After installation, you can create a configuration file:

```bash
# Create config directory
mkdir -p ~/.config/fetchx

# Generate default config
fetchx --print-config > ~/.config/fetchx/config.toml

# Edit configuration
nano ~/.config/fetchx/config.toml
```

## Uninstallation

### From Source or Cargo

```bash
sudo rm /usr/local/bin/fetchx
rm -rf ~/.config/fetchx
```

### From AUR

```bash
yay -R fetchx
# or
paru -R fetchx
```

### From Homebrew

```bash
brew uninstall fetchx
```

### From .deb

```bash
sudo dpkg -r fetchx
```

## System Requirements

- **Operating System**: Linux, macOS, BSD, Windows (WSL)
- **Memory**: ~5MB RAM
- **Disk Space**: ~10MB
- **Dependencies**: None (statically linked binary)

## Build Requirements (for building from source)

- Rust 1.70 or later
- Cargo (comes with Rust)
- Git (for cloning repository)

### Install Rust

```bash
# Using rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

## Troubleshooting

### "Command not found"

Make sure `/usr/local/bin` is in your PATH:

```bash
export PATH=$PATH:/usr/local/bin
```

### Permission denied

Make the binary executable:

```bash
chmod +x /usr/local/bin/fetchx
```

### Colors not showing

Ensure your terminal supports colors and try:

```bash
fetchx --no-color  # Disable colors
```

### ASCII art not aligned

Adjust the gap setting:

```bash
fetchx --gap 5  # Increase gap
fetchx --gap 1  # Decrease gap
```

## Performance Tips

1. **Use release build** for best performance:

   ```bash
   cargo build --release
   ```

2. **Disable unnecessary info** in config to speed up:

   ```toml
   show_resolution = false
   show_de = false
   show_wm = false
   ```

3. **Use ASCII backend** instead of image backends for faster startup.

## Updates

### From Cargo

```bash
cargo install --force fetchx
```

### From AUR

```bash
yay -Syu fetchx
```

### From Source

```bash
git pull
cargo build --release
sudo cp target/release/fetchx /usr/local/bin/
```

## Getting Help

- **Documentation**: `fetchx --help`
- **Config example**: `fetchx --print-config`
- **Issues**: https://github.com/swadhinbiswas/fetchx/issues
- **Discussions**: https://github.com/swadhinbiswas/fetchx/discussions

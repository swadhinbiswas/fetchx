//! Configuration module for FetchX

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Color configuration — either "distro" auto-detection or explicit color numbers.
/// Neofetch color numbers: 0=black, 1=red, 2=green, 3=yellow, 4=blue,
/// 5=magenta, 6=cyan, 7=white, 8-255=256-color palette.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorConfig {
    /// Use distro-derived colors automatically
    Auto(String), // "distro"
    /// Explicit color numbers
    Numbers(Vec<u8>),
}

impl Default for ColorConfig {
    fn default() -> Self {
        ColorConfig::Auto("distro".to_string())
    }
}

impl ColorConfig {
    #[allow(dead_code)]
    pub fn is_distro(&self) -> bool {
        matches!(self, ColorConfig::Auto(s) if s == "distro")
    }
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // ── Display options ──────────────────────────────────────────

    #[serde(default = "default_false")]
    pub no_color: bool,
    #[serde(default = "default_true")]
    pub bold: bool,
    #[serde(default = "default_separator")]
    pub separator: String,

    // ── Text colors ──────────────────────────────────────────────
    // "distro" = auto from ASCII art colors (default)
    // [6, 6, 7, 1, 7, 7] = explicit: [title, @, underline, subtitle, colon, info]
    #[serde(default)]
    pub colors: ColorConfig,

    // ── ASCII / Logo options ─────────────────────────────────────

    /// Image backend: "ascii", "kitty", "sixel", "chafa", "off"
    #[serde(default = "default_ascii")]
    pub image_backend: String,

    /// Image source: "auto", "ascii", "wallpaper", "/path/to/image"
    #[serde(default = "default_auto")]
    pub image_source: String,

    /// Which distro's ASCII art to display: "auto", "arch", "ubuntu", etc.
    #[serde(default = "default_auto")]
    pub ascii_distro: String,

    /// Path to a custom ASCII art file (overrides ascii_distro)
    /// The file can contain {c1}..{c6} color placeholders
    #[serde(default)]
    pub ascii_file: Option<String>,

    /// Colors for ASCII art: "distro" or explicit [6, 6, 7, 1]
    #[serde(default)]
    pub ascii_colors: ColorConfig,

    /// Bold the ASCII art
    #[serde(default = "default_true")]
    pub ascii_bold: bool,

    // ── Image options ────────────────────────────────────────────

    /// Path to custom image to display (for kitty/sixel/chafa backends)
    #[serde(default)]
    pub custom_image: Option<String>,

    /// Image size: "auto", "none", or "NNpx" / "NN%"
    #[serde(default = "default_auto")]
    pub image_size: String,

    /// Crop mode: "normal", "fit", "fill"
    #[serde(default = "default_normal")]
    pub crop_mode: String,

    /// Crop offset: "center", "north", "south", "east", "west", etc.
    #[serde(default = "default_center")]
    pub crop_offset: String,

    // ── Extra features ───────────────────────────────────────────

    /// Use emoji art instead of ASCII art
    #[serde(default = "default_false")]
    pub emoji_mode: bool,

    /// Use Nerd Font icons for info labels
    #[serde(default = "default_false")]
    pub nerd_font: bool,

    // ── Layout ───────────────────────────────────────────────────

    /// Gap between logo and info text (in chars)
    #[serde(default = "default_gap")]
    pub gap: usize,

    /// Only show the logo/ASCII art, no system info
    #[serde(default = "default_false")]
    pub logo_only: bool,

    // ── Info field toggles ───────────────────────────────────────

    #[serde(default = "default_true")]
    pub show_title: bool,
    #[serde(default = "default_true")]
    pub show_underline: bool,
    #[serde(default = "default_true")]
    pub show_os: bool,
    #[serde(default = "default_true")]
    pub show_host: bool,
    #[serde(default = "default_true")]
    pub show_kernel: bool,
    #[serde(default = "default_true")]
    pub show_uptime: bool,
    #[serde(default = "default_true")]
    pub show_packages: bool,
    #[serde(default = "default_true")]
    pub show_shell: bool,
    #[serde(default = "default_true")]
    pub show_resolution: bool,
    #[serde(default = "default_true")]
    pub show_de: bool,
    #[serde(default = "default_true")]
    pub show_wm: bool,
    #[serde(default = "default_true")]
    pub show_wm_theme: bool,
    #[serde(default = "default_true")]
    pub show_theme: bool,
    #[serde(default = "default_true")]
    pub show_icons: bool,
    #[serde(default = "default_true")]
    pub show_terminal: bool,
    #[serde(default = "default_true")]
    pub show_term_font: bool,
    #[serde(default = "default_true")]
    pub show_cpu: bool,
    #[serde(default = "default_true")]
    pub show_gpu: bool,
    #[serde(default = "default_true")]
    pub show_memory: bool,
    #[serde(default = "default_false")]
    pub show_disk: bool,
    #[serde(default = "default_false")]
    pub show_battery: bool,
    #[serde(default = "default_false")]
    pub show_local_ip: bool,
    #[serde(default = "default_false")]
    pub show_public_ip: bool,
    #[serde(default = "default_false")]
    pub show_locale: bool,
    #[serde(default = "default_false")]
    pub show_song: bool,
    #[serde(default = "default_false")]
    pub show_users: bool,
    #[serde(default = "default_true")]
    pub show_colors: bool,

    // ── Progress bars ────────────────────────────────────────────

    /// Show progress bar next to memory percentage
    #[serde(default = "default_false")]
    pub memory_bar: bool,

    /// Show progress bar next to disk percentage
    #[serde(default = "default_false")]
    pub disk_bar: bool,

    /// Width of progress bars (in chars)
    #[serde(default = "default_bar_width")]
    pub bar_width: usize,

    // ── Color blocks ─────────────────────────────────────────────

    #[serde(default = "default_block_range")]
    pub block_range: (u8, u8),
    #[serde(default = "default_block_width")]
    pub block_width: usize,
    #[serde(default = "default_block_height")]
    pub block_height: usize,

    // ── Underline ────────────────────────────────────────────────

    /// Underline character (neofetch defaults to "-")
    #[serde(default = "default_underline_char")]
    pub underline_char: String,

    // ── Misc ─────────────────────────────────────────────────────

    /// Stdout mode — disable colors & image backend (for piping)
    #[serde(default = "default_false")]
    pub stdout: bool,
}

fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
fn default_ascii() -> String {
    "ascii".to_string()
}
fn default_auto() -> String {
    "auto".to_string()
}
fn default_normal() -> String {
    "normal".to_string()
}
fn default_center() -> String {
    "center".to_string()
}
fn default_gap() -> usize {
    3
}
fn default_separator() -> String {
    ": ".to_string()
}
fn default_block_range() -> (u8, u8) {
    (0, 15)
}
fn default_block_width() -> usize {
    3
}
fn default_block_height() -> usize {
    1
}
fn default_bar_width() -> usize {
    15
}
fn default_underline_char() -> String {
    "-".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            no_color: false,
            bold: true,
            separator: ": ".to_string(),
            colors: ColorConfig::default(),
            image_backend: "auto".to_string(),
            image_source: "auto".to_string(),
            ascii_distro: "auto".to_string(),
            ascii_file: None,
            ascii_colors: ColorConfig::default(),
            ascii_bold: true,
            custom_image: None,
            image_size: "auto".to_string(),
            crop_mode: "normal".to_string(),
            crop_offset: "center".to_string(),
            emoji_mode: false,
            nerd_font: false,
            gap: 3,
            logo_only: false,
            show_title: true,
            show_underline: true,
            show_os: true,
            show_host: true,
            show_kernel: true,
            show_uptime: true,
            show_packages: true,
            show_shell: true,
            show_resolution: true,
            show_de: true,
            show_wm: true,
            show_wm_theme: true,
            show_theme: true,
            show_icons: true,
            show_terminal: true,
            show_term_font: true,
            show_cpu: true,
            show_gpu: true,
            show_memory: true,
            show_disk: false,
            show_battery: false,
            show_local_ip: false,
            show_public_ip: false,
            show_locale: false,
            show_song: false,
            show_users: false,
            show_colors: true,
            memory_bar: false,
            disk_bar: false,
            bar_width: 15,
            block_range: (0, 15),
            block_width: 3,
            block_height: 1,
            underline_char: "-".to_string(),
            stdout: false,
        }
    }
}

impl Config {
    /// Get the path to the config file.
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("fetchx")
            .join("config.toml")
    }

    /// Load config from file, or return defaults if not found.
    pub fn load_or_default() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                match toml::from_str::<Config>(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!(
                            "\x1b[33m[fetchx] Warning: config parse error: {}\x1b[0m",
                            e
                        );
                        eprintln!(
                            "\x1b[33m[fetchx] Using default config. Fix: {}\x1b[0m",
                            path.display()
                        );
                    }
                }
            }
        }
        Self::default()
    }

    /// Return the default config as a TOML string.
    #[allow(dead_code)]
    pub fn default_toml() -> String {
        toml::to_string_pretty(&Config::default()).unwrap_or_else(|_| "# error".to_string())
    }

    /// Return the full documented example config.
    pub fn example_config() -> String {
        EXAMPLE_CONFIG.to_string()
    }

    /// Create the config file with example content if it doesn't exist.
    pub fn create_default_config() -> std::io::Result<PathBuf> {
        let path = Self::config_path();
        if !path.exists() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, EXAMPLE_CONFIG)?;
        }
        Ok(path)
    }
}

/// Full documented example configuration
const EXAMPLE_CONFIG: &str = r##"# =============================================================================
# FetchX Configuration File
# Location: ~/.config/fetchx/config.toml
#
# FetchX is a fast, modern system info tool (neofetch clone in Rust).
# All options have sensible defaults — only set what you want to change.
# =============================================================================

# ─── Display Options ─────────────────────────────────────────────────────────

# Disable all colors
# no_color = false

# Enable bold text
# bold = true

# Separator between label and value
# separator = ": "


# ─── Text Colors ─────────────────────────────────────────────────────────────
# Either "distro" for automatic color matching, or an array of 6 color numbers:
#   [title, @, underline, subtitle, colon, info]
#
# Color numbers: 0=black, 1=red, 2=green, 3=yellow, 4=blue,
#                5=magenta, 6=cyan, 7=white, 8-255=256-color palette
#
# Examples:
#   colors = "distro"             # auto-detect from distro (default)
#   colors = [4, 6, 1, 8, 8, 6]  # blue title, cyan @, red underline, etc.
#   colors = [6, 6]               # cyan title & subtitle, rest default
colors = "distro"


# ─── ASCII Art / Logo ────────────────────────────────────────────────────────

# Image backend: "auto", "ascii", "kitty", "sixel", "chafa", "w3m", "iterm2", "off"
# "auto" = auto-detect based on terminal capabilities (RECOMMENDED)
# "ascii" = ASCII art logo (works everywhere)
# "kitty" = Kitty terminal graphics protocol
# "sixel" = Sixel graphics (requires img2sixel or chafa)
# "chafa" = Chafa text-based image conversion (best fallback)
# "w3m" = w3m image display (X11 only)
# "iterm2" = iTerm2 inline images
# "off" = no logo/image
image_backend = "auto"

# Image source: "auto", "ascii", "/path/to/image.png"
# "auto" = automatically use cached image from API (downloads new ones every 6 hours)
#          Shows previous image while downloading next one in background
# "ascii" = only use ASCII art, never fetch images
# "/path/to/image.png" = use specific image file
image_source = "auto"

# Which distro's ASCII art to display (when using ASCII backend)
# Values: "auto", "arch", "ubuntu", "debian", "fedora", "gentoo", "nixos",
#         "manjaro", "void", "pop", "cachyos", "endeavouros", "artix", "kali",
#         "opensuse", "linuxmint", "alpine", "garuda", "zorin", "elementary",
#         "rocky", "alma", "slackware", "centos", "solus", "deepin", "mx",
#         "raspbian", "freebsd", "openbsd", "macos", "windows", "android"
ascii_distro = "auto"

# Path to a custom ASCII art file (overrides ascii_distro when set)
# The file can use {c1}..{c6} color placeholders, one line per line of art.
# Example: ascii_file = "/home/user/.config/fetchx/my_logo.txt"
# ascii_file = null

# Colors for ASCII art: "distro" or explicit color numbers
# Examples:
#   ascii_colors = "distro"       # use distro's default colors
#   ascii_colors = [4, 6, 1]      # custom colors for {c1}, {c2}, {c3}
ascii_colors = "distro"

# Bold the ASCII art
# ascii_bold = true


# ─── Image Options (for kitty/sixel/chafa backends) ──────────────────────────

# Path to a custom image file
# custom_image = "/path/to/image.png"

# Image size: "auto", "none", or dimensions like "300px", "50%"
# image_size = "auto"

# Crop mode: "normal", "fit", "fill"
# crop_mode = "normal"

# Crop offset: "center", "north", "south", "east", "west",
#              "northwest", "northeast", "southwest", "southeast"
# crop_offset = "center"


# ─── Extra Features ──────────────────────────────────────────────────────────

# Use emoji art instead of ASCII art
# emoji_mode = false

# Use Nerd Font icons for info labels (requires a Nerd Font terminal font)
# nerd_font = false


# ─── Layout ──────────────────────────────────────────────────────────────────

# Gap between logo/image and info text (characters)
# gap = 3

# Show only the logo, hide system info
# logo_only = false


# ─── Info Fields ─────────────────────────────────────────────────────────────
# Toggle which info lines to display

show_title = true
show_underline = true
show_os = true
show_host = true
show_kernel = true
show_uptime = true
show_packages = true
show_shell = true
show_resolution = true
show_de = true
show_wm = true
show_wm_theme = true
show_theme = true
show_icons = true
show_terminal = true
show_term_font = true
show_cpu = true
show_gpu = true
show_memory = true
show_disk = false
show_battery = false
show_local_ip = false
show_public_ip = false
show_locale = false
show_song = false
show_users = false
show_colors = true


# ─── Progress Bars ──────────────────────────────────────────────────────────
# Show progress bars alongside memory/disk usage

# Show a progress bar next to memory info
# memory_bar = false

# Show a progress bar next to disk info
# disk_bar = false

# Width of progress bars (in characters)
# bar_width = 15


# ─── Color Blocks ───────────────────────────────────────────────────────────

# Range of colors to display in color blocks
# Default shows all 16 standard terminal colors (0-15)
# block_range = [0, 15]

# Width of each color block (in spaces)
# block_width = 3

# Height of color blocks (in lines)
# block_height = 1


# ─── Underline ───────────────────────────────────────────────────────────────

# Character used for the title underline
# underline_char = "-"


# ─── Misc ────────────────────────────────────────────────────────────────────

# Stdout mode: disable colors and image (useful for piping)
# stdout = false
"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.separator, ": ");
        assert!(!config.no_color);
        assert!(config.bold);
        assert!(config.show_title);
        assert!(config.show_os);
        assert!(config.show_cpu);
        assert!(config.show_memory);
        assert!(!config.show_disk);
        assert!(!config.show_battery);
        assert!(!config.show_song);
        assert!(!config.show_users);
        assert!(!config.memory_bar);
        assert!(!config.disk_bar);
        assert_eq!(config.bar_width, 15);
        assert_eq!(config.gap, 3);
        assert_eq!(config.image_backend, "ascii");
        assert_eq!(config.block_range, (0, 15));
    }

    #[test]
    fn test_color_config_distro() {
        let cc = ColorConfig::Auto("distro".to_string());
        assert!(cc.is_distro());
    }

    #[test]
    fn test_color_config_numbers() {
        let cc = ColorConfig::Numbers(vec![1, 2, 3]);
        assert!(!cc.is_distro());
    }

    #[test]
    fn test_config_parse_minimal_toml() {
        let toml_str = r#"
            no_color = true
            separator = " => "
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.no_color);
        assert_eq!(config.separator, " => ");
        // Defaults should fill in
        assert!(config.show_title);
        assert_eq!(config.gap, 3);
    }

    #[test]
    fn test_config_parse_colors_distro() {
        let toml_str = r#"colors = "distro""#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.colors.is_distro());
    }

    #[test]
    fn test_config_parse_colors_numbers() {
        let toml_str = r#"colors = [4, 6, 1, 8]"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        match config.colors {
            ColorConfig::Numbers(nums) => assert_eq!(nums, vec![4, 6, 1, 8]),
            _ => panic!("Expected Numbers variant"),
        }
    }

    #[test]
    fn test_config_parse_bar_settings() {
        let toml_str = r#"
            memory_bar = true
            disk_bar = true
            bar_width = 20
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.memory_bar);
        assert!(config.disk_bar);
        assert_eq!(config.bar_width, 20);
    }

    #[test]
    fn test_config_parse_song_users() {
        let toml_str = r#"
            show_song = true
            show_users = true
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.show_song);
        assert!(config.show_users);
    }

    #[test]
    fn test_example_config_is_valid_toml() {
        let result = toml::from_str::<Config>(EXAMPLE_CONFIG);
        assert!(result.is_ok(), "EXAMPLE_CONFIG is not valid: {:?}", result.err());
    }

    #[test]
    fn test_config_serialize_roundtrip() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.separator, config.separator);
        assert_eq!(parsed.gap, config.gap);
        assert_eq!(parsed.no_color, config.no_color);
    }
}
// test: default config

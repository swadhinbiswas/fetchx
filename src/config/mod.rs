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

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


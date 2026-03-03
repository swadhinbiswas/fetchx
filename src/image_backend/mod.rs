//! Image backend module for FetchX
//!
//! Supports: kitty graphics protocol, sixel (via external tool), chafa, and "off".

use base64::Engine;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Image backend types
#[derive(Debug, Clone, PartialEq)]
pub enum Backend {
    Ascii,
    Kitty,
    Sixel,
    Chafa,
    W3m,
    Iterm2,
    Off,
}

impl Backend {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "kitty" => Backend::Kitty,
            "sixel" => Backend::Sixel,
            "chafa" => Backend::Chafa,
            "w3m" | "w3mimgdisplay" => Backend::W3m,
            "iterm2" | "iterm" => Backend::Iterm2,
            "off" => Backend::Off,
            "auto" => Backend::Ascii, // Default: auto-detection is handled in main.rs
            _ => Backend::Ascii,
        }
    }

    /// Check if the terminal supports this backend.
    #[allow(dead_code)]
    pub fn is_supported(&self) -> bool {
        match self {
            Backend::Ascii | Backend::Off => true,
            Backend::Kitty => {
                // Kitty is supported if TERM contains "kitty" or KITTY_PID is set
                std::env::var("KITTY_PID").is_ok()
                    || std::env::var("TERM")
                        .unwrap_or_default()
                        .contains("kitty")
                    || std::env::var("TERM_PROGRAM")
                        .unwrap_or_default()
                        .contains("kitty")
            }
            Backend::Sixel => {
                // Check if img2sixel or ImageMagick convert is available
                Command::new("img2sixel")
                    .arg("--version")
                    .output()
                    .is_ok()
            }
            Backend::Chafa => Command::new("chafa")
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false),
            Backend::W3m => w3m_img_path().is_some(),
            Backend::Iterm2 => {
                std::env::var("TERM_PROGRAM")
                    .unwrap_or_default()
                    .contains("iTerm")
            }
        }
    }
}

/// Result of image rendering — returns lines for side-by-side layout
/// or renders directly (kitty/sixel print to stdout).
#[allow(dead_code)]
pub enum ImageResult {
    /// Image was rendered inline (kitty/sixel) — gives placeholder lines for layout
    InlineRendered {
        /// Number of terminal rows the image occupies
        height_rows: usize,
        /// Width in columns
        width_cols: usize,
    },
    /// Image converted to text lines (chafa)
    TextLines(Vec<String>),
    /// Backend not available or failed
    Failed(String),
}

/// Extract frames from an animated GIF (simplified: returns first frame for now)
fn extract_gif_frames(image_path: &str) -> Result<Vec<image::DynamicImage>, String> {
    let path = Path::new(image_path);

    // Check if file has .gif extension
    if !image_path.ends_with(".gif") && !image_path.ends_with(".GIF") {
        return Ok(vec![]);
    }

    match image::open(path) {
        Ok(img) => {

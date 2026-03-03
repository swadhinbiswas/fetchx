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
            // For now, return single frame (first frame of GIF)
            // Full animation support would require the 'gif' crate
            Ok(vec![img])
        }
        Err(e) => Err(format!("Failed to open GIF: {}", e)),
    }
}

/// Display a single image frame using kitty graphics protocol
fn display_kitty_frame(
    img: &image::DynamicImage,
    max_cols: usize,
    max_rows: usize,
) -> Result<(usize, usize), String> {
    // Calculate dimensions — fit within max_cols x max_rows
    let (orig_w, orig_h) = (img.width() as f64, img.height() as f64);
    let cell_w = 8.0_f64;
    let cell_h = 16.0_f64;

    let max_px_w = max_cols as f64 * cell_w;
    let max_px_h = max_rows as f64 * cell_h;

    let scale = (max_px_w / orig_w).min(max_px_h / orig_h).min(1.0);
    let target_w = (orig_w * scale) as u32;
    let target_h = (orig_h * scale) as u32;

    let resized = img.resize(target_w, target_h, image::imageops::FilterType::Lanczos3);
    let rgba = resized.to_rgba8();
    let raw = rgba.as_raw();

    let encoded = base64::engine::general_purpose::STANDARD.encode(raw);
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let chunk_size = 4096;
    let chunks: Vec<&str> = encoded
        .as_bytes()
        .chunks(chunk_size)
        .map(|c| std::str::from_utf8(c).unwrap_or(""))
        .collect();

    for (i, chunk) in chunks.iter().enumerate() {
        let more = if i < chunks.len() - 1 { 1 } else { 0 };
        if i == 0 {
            write!(
                out,
                "\x1b_Ga=T,f=32,s={},v={},m={};{}\x1b\\",
                target_w, target_h, more, chunk
            )
            .ok();

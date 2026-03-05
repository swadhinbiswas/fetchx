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
        } else {
            write!(out, "\x1b_Gm={};{}\x1b\\", more, chunk).ok();
        }
    }

    let display_w = (target_w as f64 / cell_w).ceil() as usize;
    let display_h = (target_h as f64 / cell_h).ceil() as usize;

    Ok((display_w, display_h))
}

/// Display an image using the kitty graphics protocol.
/// Returns the dimensions used.
pub fn display_kitty(
    image_path: &str,
    max_cols: usize,
    max_rows: usize,
) -> Result<(usize, usize), String> {
    let path = Path::new(image_path);
    if !path.exists() {
        return Err(format!("Image not found: {}", image_path));
    }

    // Check if it's an animated GIF
    if let Ok(frames) = extract_gif_frames(image_path) {
        if frames.len() > 1 {
            // Display first frame for now (full animation support requires terminal refresh loop)
            return display_kitty_frame(&frames[0], max_cols, max_rows);
        }
    }

    let img = image::open(path).map_err(|e| format!("Failed to open image: {}", e))?;

    // Calculate dimensions — fit within max_cols x max_rows
    // Assume ~2:1 character aspect ratio (chars are taller than wide)
    let (orig_w, orig_h) = (img.width() as f64, img.height() as f64);
    let cell_w = 8.0_f64; // approximate pixel width of a terminal cell
    let cell_h = 16.0_f64; // approximate pixel height of a terminal cell

    let max_px_w = max_cols as f64 * cell_w;
    let max_px_h = max_rows as f64 * cell_h;

    let scale = (max_px_w / orig_w).min(max_px_h / orig_h).min(1.0);
    let target_w = (orig_w * scale) as u32;
    let target_h = (orig_h * scale) as u32;

    let resized = img.resize(target_w, target_h, image::imageops::FilterType::Lanczos3);
    let rgba = resized.to_rgba8();
    let raw = rgba.as_raw();

    // Kitty graphics protocol: transmit in chunks
    // Format: \x1b_Ga=T,f=32,s=<width>,v=<height>,m=1;<base64>\x1b\\
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
                resized.width(),
                resized.height(),
                more,
                chunk
            )
            .ok();
        } else {
            write!(out, "\x1b_Gm={};{}\x1b\\", more, chunk).ok();
        }
    }
    out.flush().ok();

    // Return how many terminal cells the image occupies
    let cols_used = (resized.width() as f64 / cell_w).ceil() as usize;
    let rows_used = (resized.height() as f64 / cell_h).ceil() as usize;

    Ok((cols_used, rows_used))
}

/// Display an image using chafa (converts to text-based art).
/// Returns the rendered lines.
pub fn display_chafa(
    image_path: &str,
    max_cols: usize,
    max_rows: usize,
) -> Result<Vec<String>, String> {
    let path = Path::new(image_path);
    if !path.exists() {
        return Err(format!("Image not found: {}", image_path));
    }

    let output = Command::new("chafa")
        .args([
            "--size",
            &format!("{}x{}", max_cols, max_rows),
            "--animate",
            "off",
            image_path,
        ])
        .output()
        .map_err(|e| format!("Failed to run chafa: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "chafa failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.lines().map(|l| l.to_string()).collect())
}

/// Display an image using sixel (via img2sixel).
pub fn display_sixel(
    image_path: &str,
    max_cols: usize,
    _max_rows: usize,
) -> Result<(), String> {
    let path = Path::new(image_path);
    if !path.exists() {
        return Err(format!("Image not found: {}", image_path));
    }

    // Calculate pixel width (approx 8px per col)
    let px_width = max_cols * 8;

    let output = Command::new("img2sixel")
        .args(["-w", &px_width.to_string(), image_path])
        .output()
        .map_err(|e| format!("Failed to run img2sixel: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "img2sixel failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    out.write_all(&output.stdout).ok();
    out.flush().ok();

    Ok(())
}

/// Render an image with the specified backend. Returns lines for side-by-side layout.
/// For kitty/sixel, the image is printed directly and empty placeholder lines are returned.
#[allow(dead_code)]
pub fn render_image(
    backend: &Backend,
    image_path: &str,
    max_cols: usize,
    max_rows: usize,
) -> ImageResult {
    match backend {
        Backend::Kitty => match display_kitty(image_path, max_cols, max_rows) {
            Ok((w, h)) => ImageResult::InlineRendered {
                height_rows: h,
                width_cols: w,
            },
            Err(e) => ImageResult::Failed(e),
        },
        Backend::Chafa => match display_chafa(image_path, max_cols, max_rows) {
            Ok(lines) => ImageResult::TextLines(lines),
            Err(e) => ImageResult::Failed(e),
        },
        Backend::Sixel => match display_sixel(image_path, max_cols, max_rows) {
            Ok(()) => ImageResult::InlineRendered {
                height_rows: max_rows,
                width_cols: max_cols,
            },
            Err(e) => ImageResult::Failed(e),
        },
        Backend::W3m => match display_w3m(image_path, max_cols, max_rows) {
            Ok(()) => ImageResult::InlineRendered {
                height_rows: max_rows,
                width_cols: max_cols,
            },
            Err(e) => ImageResult::Failed(e),
        },
        Backend::Iterm2 => match display_iterm2(image_path, max_cols, max_rows) {
            Ok(()) => ImageResult::InlineRendered {
                height_rows: max_rows,
                width_cols: max_cols,
            },
            Err(e) => ImageResult::Failed(e),
        },
        _ => ImageResult::Failed("Backend does not support images".to_string()),
    }
}

/// Find the w3mimgdisplay binary path.
fn w3m_img_path() -> Option<String> {
    let paths = [
        "/usr/lib/w3m/w3mimgdisplay",
        "/usr/libexec/w3m/w3mimgdisplay",
        "/usr/lib64/w3m/w3mimgdisplay",
        "/usr/local/lib/w3m/w3mimgdisplay",
    ];
    for p in &paths {
        if Path::new(p).exists() {
            return Some(p.to_string());
        }
    }
    // Try which
    if let Ok(output) = Command::new("which").arg("w3mimgdisplay").output() {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }
    None
}

/// Display an image using w3mimgdisplay.
/// w3m uses a helper binary that accepts commands on stdin:
///   0;1;x;y;w;h;sx;sy;sw;sh;path\n  (draw)
///   6;offset_x;offset_y;width;height\n  (sync)
pub fn display_w3m(
    image_path: &str,
    max_cols: usize,
    max_rows: usize,
) -> Result<(), String> {
    let path = Path::new(image_path);
    if !path.exists() {
        return Err(format!("Image not found: {}", image_path));
    }

    let w3m_bin = w3m_img_path().ok_or("w3mimgdisplay not found")?;

    // Get pixel dimensions from terminal
    let cell_w = 8;
    let cell_h = 16;
    let max_px_w = max_cols * cell_w;
    let max_px_h = max_rows * cell_h;

    // Get original image size using w3mimgdisplay (command 5)
    let size_cmd = format!("5;{}\n", image_path);
    let size_output = Command::new(&w3m_bin)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .and_then(|mut child| {
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(size_cmd.as_bytes()).ok();
            }
            child.wait_with_output()
        })
        .map_err(|e| format!("w3mimgdisplay failed: {}", e))?;

    let size_str = String::from_utf8_lossy(&size_output.stdout);
    let (orig_w, orig_h) = size_str
        .trim()
        .split_once(' ')
        .and_then(|(w, h)| {
            let w: u32 = w.parse().ok()?;
            let h: u32 = h.parse().ok()?;
            Some((w, h))
        })
        .unwrap_or((max_px_w as u32, max_px_h as u32));

    // Scale to fit
    let scale = (max_px_w as f64 / orig_w as f64)
        .min(max_px_h as f64 / orig_h as f64)
        .min(1.0);
    let target_w = (orig_w as f64 * scale) as u32;
    let target_h = (orig_h as f64 * scale) as u32;

    // Draw command: 0;1;0;0;target_w;target_h;0;0;orig_w;orig_h;path
    let draw_cmd = format!(
        "0;1;0;0;{};{};0;0;{};{};{}\n3;\n",
        target_w, target_h, orig_w, orig_h, image_path
    );

    let mut child = Command::new(&w3m_bin)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("w3mimgdisplay failed: {}", e))?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(draw_cmd.as_bytes()).ok();
    }
    child.wait().ok();

    Ok(())
}

/// Display an image using iTerm2's inline image protocol.
/// Format: \x1b]1337;File=inline=1;width=Ncells;height=Nrows;preserveAspectRatio=1:<base64>\x07
pub fn display_iterm2(
    image_path: &str,
    max_cols: usize,
    max_rows: usize,
) -> Result<(), String> {
    let path = Path::new(image_path);
    if !path.exists() {
        return Err(format!("Image not found: {}", image_path));
    }

    let file_data =
        std::fs::read(path).map_err(|e| format!("Failed to read image: {}", e))?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&file_data);
    let file_size = file_data.len();

    let stdout = io::stdout();
    let mut out = stdout.lock();

    // iTerm2 inline image protocol
    write!(
        out,
        "\x1b]1337;File=inline=1;width={};height={};size={};preserveAspectRatio=1:{}\x07",
        max_cols, max_rows, file_size, encoded
    )
    .ok();
    out.flush().ok();

    Ok(())
}

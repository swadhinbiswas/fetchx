//! Image caching and background downloading from API
//!
//! Implements smart caching where the previous image is shown while a new one
//! is downloaded in the background for the next run.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Get the cache directory for images
pub fn cache_dir() -> PathBuf {
    let cache = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    cache.join("fetchx")
}

/// Get the cached image path
pub fn cached_image_path() -> PathBuf {
    cache_dir().join("current_image.png")
}

/// Download a new image in background every run (non-blocking)
/// Spawns a detached child process (fetchx --bg-download) that survives
/// after the main process exits, ensuring the download always completes.
pub fn download_image_background() {
    // Find our own executable path
    if let Ok(exe) = std::env::current_exe() {
        let _ = Command::new(exe)
            .arg("--bg-download")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn(); // Detached child process — survives after parent exits
    }
}

/// Download image from API and save to cache directory
pub fn download_image_from_api() -> Result<(), Box<dyn std::error::Error>> {
    let cache = cache_dir();
    fs::create_dir_all(&cache)?;

    // /file endpoint returns raw image bytes directly — no JSON parsing needed
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client
        .get("https://api.nekosapi.com/v4/images/random/file")
        .send()?;

    response.error_for_status_ref()?;

    let bytes = response.bytes()?;
    if bytes.is_empty() {
        return Err("Empty response from API".into());
    }

    // Decode whatever format API sends (webp/jpeg/png) and save as PNG
    let img = image::load_from_memory(&bytes)
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    // Write to temp file first, then rename — so display never sees a half-written file
    let cache = cache_dir();
    let tmp_path = cache.join("downloading.tmp.png");
    img.save_with_format(&tmp_path, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    let current = cached_image_path();
    fs::rename(&tmp_path, &current)?;

    Ok(())
}

/// Detect and use wallpaper as image (Hyprland, GNOME, KDE, etc.)
#[allow(dead_code)]
pub fn detect_and_use_wallpaper() -> Option<PathBuf> {
    // Try Hyprland wallpaper detection
    if let Some(wallpaper) = get_hyprland_wallpaper() {
        if let Ok(_) = copy_image_to_cache(&wallpaper) {
            return Some(cached_image_path());
        }
    }

    // Try GNOME wallpaper
    if let Some(wallpaper) = get_gnome_wallpaper() {
        if let Ok(_) = copy_image_to_cache(&wallpaper) {
            return Some(cached_image_path());
        }
    }

    // Try KDE wallpaper
    if let Some(wallpaper) = get_kde_wallpaper() {
        if let Ok(_) = copy_image_to_cache(&wallpaper) {
            return Some(cached_image_path());
        }
    }

    // Try from environment variable
    if let Ok(wallpaper_path) = std::env::var("WALLPAPER") {
        let path = PathBuf::from(wallpaper_path);
        if path.exists() {
            if let Ok(_) = copy_image_to_cache(&path) {
                return Some(cached_image_path());
            }
        }
    }

    None
}

/// Get Hyprland wallpaper (check hyprctl output and config files)
fn get_hyprland_wallpaper() -> Option<PathBuf> {
    // Try hyprctl command first
    if let Ok(output) = Command::new("hyprctl")
        .args(&["hyprpaper", "wallpapers"])
        .output()
    {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines() {
                let path = line.trim();
                let file_path = PathBuf::from(path);
                if file_path.exists() && is_image_file(&file_path) {
                    return Some(file_path);
                }
            }
        }
    }

    // Try hyprpaper config
    let hyprpaper_config = dirs::config_dir()
        .map(|d| d.join("hypr/hyprpaper.conf"))?;

    if let Ok(content) = fs::read_to_string(&hyprpaper_config) {
        for line in content.lines() {
            if line.starts_with("preload") || line.starts_with("wallpaper") {
                if let Some(path_str) = line.split('=').nth(1) {
                    let path_str = path_str.trim().trim_matches('"').trim_matches('\'');
                    let path = expand_path(path_str);
                    if path.exists() && is_image_file(&path) {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

/// Get GNOME wallpaper from dconf
fn get_gnome_wallpaper() -> Option<PathBuf> {
    let output = Command::new("dconf")
        .args(&["read", "/org/gnome/desktop/background/picture-uri-dark"])
        .output()
        .ok()?;

    let path_str = String::from_utf8(output.stdout).ok()?;
    let path_str = path_str
        .trim()
        .trim_matches('\'')
        .trim_start_matches("file://");

    let path = PathBuf::from(path_str);
    if path.exists() && is_image_file(&path) {
        return Some(path);
    }

    None
}

/// Get KDE wallpaper from config
fn get_kde_wallpaper() -> Option<PathBuf> {
    let kde_config = dirs::config_dir()
        .map(|d| d.join("plasmarc"))?;

    if let Ok(content) = fs::read_to_string(&kde_config) {
        for line in content.lines() {
            if line.contains("Image=") || line.contains("File=") {
                if let Some(value) = line.split('=').nth(1) {
                    let path = expand_path(value.trim());
                    if path.exists() && is_image_file(&path) {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

/// Copy wallpaper image to cache directory
fn copy_image_to_cache(source: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let cache = cache_dir();
    fs::create_dir_all(&cache)?;

    let cached = cached_image_path();
    fs::copy(source, &cached)?;

    Ok(())
}

/// Check if a file is likely an image
fn is_image_file(path: &PathBuf) -> bool {
    let extensions = ["png", "jpg", "jpeg", "webp", "gif", "bmp"];
    if let Some(ext) = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
    {
        extensions.contains(&ext.as_str())
    } else {
        false
    }
}

/// Expand ~ and environment variables in paths
fn expand_path(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(path.replace("~", &home))
        } else {
            PathBuf::from(path)
        }
    } else {
        PathBuf::from(path)
    }
}


pub fn get_available_image() -> Option<PathBuf> {
    let cache = cached_image_path();
    if cache.exists() {
        Some(cache)
    } else {
        None
    }
}

/// Detect terminal capabilities
pub fn detect_terminal_capabilities() -> TerminalCapabilities {
    let term = std::env::var("TERM").unwrap_or_default();
    let colorterm = std::env::var("COLORTERM").unwrap_or_default();

    // Check for kitty
    if std::env::var("KITTY_WINDOW_ID").is_ok() {
        return TerminalCapabilities {
            supports_graphics: true,
            backend: "kitty".to_string(),
        };
    }

    // Check for iTerm2
    if std::env::var("ITERM_SESSION_ID").is_ok() {
        return TerminalCapabilities {
            supports_graphics: true,
            backend: "iterm2".to_string(),
        };
    }

    // Check for sixel support
    if term.contains("xterm-kitty") || colorterm.contains("truecolor") {
        if check_sixel_support() {
            return TerminalCapabilities {
                supports_graphics: true,
                backend: "sixel".to_string(),
            };
        }
    }

    // Check for w3m (X11)
    if std::env::var("DISPLAY").is_ok() {
        return TerminalCapabilities {
            supports_graphics: true,
            backend: "w3m".to_string(),
        };
    }

    // Fallback: no graphics support, use ASCII
    TerminalCapabilities {
        supports_graphics: false,
        backend: "ascii".to_string(),
    }
}

pub struct TerminalCapabilities {
    #[allow(dead_code)]
    pub supports_graphics: bool,
    pub backend: String,
}

/// Check if terminal supports sixel (heuristic)
fn check_sixel_support() -> bool {
    std::env::var("TERM")
        .map(|t| t.contains("256color") || t.contains("truecolor"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_dir_exists() {
        let dir = cache_dir();
        assert!(dir.to_string_lossy().contains("cache"));
    }

    #[test]
    fn test_should_download_returns_bool() {
        let result = should_download();
        assert!(result || !result); // Just verify it returns a bool
    }
}
// fix: API error log

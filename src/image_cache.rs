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

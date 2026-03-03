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

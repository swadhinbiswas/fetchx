//! System information module for FetchX
//!
//! Gathers all system info fields, mirroring neofetch's detection logic.

use crate::utils::{command_exists, env_or, read_file, read_first_line, run_cmd};
use std::path::Path;

/// All system information fields.
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub username: String,
    pub hostname: String,
    pub os: String,
    pub distro_id: String,
    pub host: String,
    pub kernel: String,
    pub uptime: String,
    pub packages: String,
    pub shell: String,
    pub resolution: String,
    pub de: String,
    pub wm: String,
    pub wm_theme: String,
    pub theme: String,
    pub icons: String,
    pub terminal: String,
    pub term_font: String,
    pub cpu: String,
    pub gpu: Vec<String>,
    pub memory: String,
    pub memory_percent: f64,
    pub disk: String,
    pub disk_percent: f64,
    pub battery: String,
    pub local_ip: String,
    pub public_ip: String,
    pub locale: String,
    pub song: String,
    pub users: String,
}

impl SystemInfo {
    /// Collect all system information using parallel threads for speed.
    pub fn collect() -> Self {
        use std::thread;

        let distro_id = detect_distro_id();
        let did = distro_id.clone();

        // Spawn threads for slow/independent operations
